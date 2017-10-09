#[macro_use]
extern crate glium;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::net::UdpSocket;
use std::collections::VecDeque;
use glium::{glutin, Surface};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::path::Path;
use std::sync::{Mutex, Arc};

mod arduino;
mod render;
mod movement;
mod clouds;

use arduino::Port;

use movement::Vec3;
use movement::max;

use clouds::{Cloud, ColourCloud, CloudSet, Channel};

use render::Colour;

const TOTAL: usize = 50;

fn create_clouds() -> Vec<Cloud> {
    let p = Path::new("data/storm.txt");
    let storm = clouds::load(&p, 0, 220);
    let mut patterns: Vec<Cloud> = Vec::new();
    match storm {
        Some(s) => patterns.push(s),
        None => println!("Failed to create clouds"),
    }
    patterns
}

fn create_colour_clouds() -> Vec<ColourCloud>{
    let mut patterns: Vec<ColourCloud> = Vec::new();
    let settings = vec![
    CloudSet{p: Path::new("data/sunrise_long.json"), mood: 0, channel: Channel::One} ,
    CloudSet{p: Path::new("data/sunrise.json"), mood: 20, channel: Channel::Two} ,
    CloudSet{p: Path::new("data/sky.json"), mood: 100, channel: Channel::One} ,
    CloudSet{p: Path::new("data/fog.json"), mood: 150, channel: Channel::Two} ,
    CloudSet{p: Path::new("data/sunset.json"), mood: 200, channel: Channel::Two} ,
    CloudSet{p: Path::new("data/storm.json"), mood: 255, channel: Channel::One} ,
        ];
    for s in settings{
        match clouds::load_json(s){
            Some(cc) => patterns.push(cc),
            None => println!("Failed to create colour clouds"),
        }
    }
    patterns
}

fn run_clouds(mut patterns: Vec<Cloud>, tx: Sender<String>, j: Arc< Mutex<f32> >){
    // To make 24fps
    let speed = std::time::Duration::from_millis(41);
    let mut last = std::time::Instant::now();
    loop{
        while last.elapsed() < speed{
            std::thread::sleep( std::time::Duration::from_millis(5) );
        }
        for mut p in &mut patterns{
            let mut colour = clouds::cloud_to_light(&mut p);
            let mut jerk = j.lock().unwrap();
            println!("Jerk: {}", *jerk);
            colour.scale( *jerk );
            let msg = arduino::light_to_msg(&colour, 0);
            tx.send(msg);
            let msg = arduino::light_to_msg(&colour, 1);
            tx.send(msg);
        }
        last = std::time::Instant::now();
    }
}

fn run_colour_clouds(mut patterns: Vec<ColourCloud>, tx: Sender<String>, j: Arc< Mutex<f32> >){
    // To make 24fps
    let speed = std::time::Duration::from_millis(41);
    let mut last = std::time::Instant::now();
    let mut colour_tots: [Colour<i16>; 2] = [
        Colour{r: 0, g: 0, b: 0},
        Colour{r: 0, g: 0, b: 0} ];
    loop{
        colour_tots[0] = Colour{r: 0, g: 0, b: 0};
        colour_tots[1] = Colour{r: 0, g: 0, b: 0};
        let mut jerk = j.lock().unwrap();
        println!("Jerk: {}", *jerk);
        for mut c in &mut patterns{
            let channel = match c.channel {
                Channel::One => 0,
                Channel::Two => 1,
            };
            let mut colour = clouds::cloud_to_colour(&mut c, *jerk);
            colour_tots[channel] += colour;
        }
        // Skew to compensate for voltage
        for i in 0..2{
            colour_tots[i].g = (colour_tots[i].g as f32 * 0.8) as i16;
            colour_tots[i].b = (colour_tots[i].b as f32 * 0.8) as i16;
        }
        while last.elapsed() < speed{
            std::thread::sleep( std::time::Duration::from_millis(5) );
        }
        let msg = arduino::light_to_msg(&colour_tots[0], 0);
        tx.send(msg);
        let msg = arduino::light_to_msg(&colour_tots[1], 1);
        tx.send(msg);
        last = std::time::Instant::now();
    }
}

fn main() {
    let (display, mut events_loop) = render::init();

    let mut patterns = create_colour_clouds();

    let mut socket = UdpSocket::bind("0.0.0.0:44444").unwrap();

    let mut port = arduino::open();

    let (sender_jerk, recv) = mpsc::channel();

    let average_jerk = Arc::new( Mutex::new(0.0) );

    std::thread::spawn(move ||{
        match port {
            Port::Open(mut p) => arduino::interact(&mut p, recv).unwrap(),
            Port::Dummy =>(),
        };
    });

    let aj2 = average_jerk.clone();
    std::thread::spawn(move ||{
        run_colour_clouds( patterns, sender_jerk, aj2);
    });

    let mut count = 0;
    
    // read from the socket
    let mut buf = [0; 100];
    let mut accels: VecDeque<Vec3> = VecDeque::with_capacity(TOTAL);

    const J_BUFF_LEN: usize = 100;
    let mut jerks: VecDeque<f32> = VecDeque::with_capacity(J_BUFF_LEN);

    const START_SIZE: usize = 1000;
    const MIN_BUFFER: f32 = 2.0;
    let mut start_total = Vec3{x: 0.0, y: 0.0, z: 0.0};
    for i in 0..START_SIZE {
        let accel = movement::read(&mut buf, &mut socket);
        start_total += accel;
    }
    start_total.scale(1.0 / START_SIZE as f32);
    let min_accel = (start_total.x + start_total.y + start_total.z) / 3.0 + MIN_BUFFER;

    loop {
        let accel = movement::read(&mut buf, &mut socket);

        // A minimum acceleration
        if accel.x > min_accel || accel.y > min_accel || accel.z > min_accel{
            accels.push_back(accel);
            if accels.len() >= TOTAL{
                accels.pop_front();
            }
        }

        let mut dj_total = movement::extract_jerk(&accels);

        dj_total = movement::clamp_jerk(&dj_total);
        
        let jerk = max(dj_total.x, max(dj_total.y, dj_total.z) );
        jerks.push_back(jerk);
        if jerks.len() >= J_BUFF_LEN{
            jerks.pop_front();
        }

        let colour = movement::average(&jerks);
        *average_jerk.lock().unwrap() = colour;

        let mut target = display.draw();
        target.clear_color(colour, colour, colour, 1.0);
        target.finish().unwrap();

        if count >= 50 {
            /*
            let msg = movement::jerk_to_light(colour, 0);
            sender_jerk.send(msg);
            let msg = movement::jerk_to_light(colour, 1);
            sender_jerk.send(msg);
            */
            count = 0;
        }


        if render::events(&mut events_loop) {
            break;
        };
        count += 1;
    }
}
