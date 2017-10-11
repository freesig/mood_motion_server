#[macro_use]
extern crate glium;
extern crate glium_text_rusttype as glium_text;

extern crate cgmath;

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
mod buffer;
mod font;

use arduino::Port;

use movement::Vec3;
use movement::max;

use clouds::{Cloud, ColourCloud, CloudSet, Channel};

use render::Colour;

use buffer::Buffer;

use font::Text;

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
    CloudSet{p: Path::new("data/sky.json"), mood: 0, channel: Channel::One} ,
    CloudSet{p: Path::new("data/fog.json"), mood: 40, channel: Channel::Two} ,
    CloudSet{p: Path::new("data/sunrise_long.json"), mood: 100, channel: Channel::One} ,
    CloudSet{p: Path::new("data/sunrise.json"), mood: 125, channel: Channel::Two} ,
    CloudSet{p: Path::new("data/sunset.json"), mood: 150, channel: Channel::One} ,
    CloudSet{p: Path::new("data/storm.json"), mood: 255, channel: Channel::One} ,
    CloudSet{p: Path::new("data/storm2.json"), mood: 225, channel: Channel::One} ,
    CloudSet{p: Path::new("data/storm_short.json"), mood: 255, channel: Channel::Two} ,
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
        //println!("Jerk: {}", *jerk);
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

fn pre_balance(mut buf: &mut[u8], mut socket: &mut UdpSocket) -> f32 {
    const START_SIZE: usize = 500;
    const MIN_BUFFER: f32 = 5.5;
    let mut start_total = Vec3{x: 0.0, y: 0.0, z: 0.0};
    for i in 0..START_SIZE {
        let accel = movement::read(&mut buf, &mut socket);
        start_total += accel;
    }
    start_total.scale(1.0 / START_SIZE as f32);
    let min_accel = max( start_total.x, max(start_total.y, start_total.z) ) + MIN_BUFFER;
    min_accel
}

fn gather_accels(mut socket: UdpSocket, dj: Arc< Mutex<Vec3> >, mut min_accel: f32, min_recv: Receiver<f32>){
    let mut accels: VecDeque<Vec3> = VecDeque::with_capacity(TOTAL);
    let mut buf = [0; 100];
    loop{
        let accel = movement::read(&mut buf, &mut socket);
        match min_recv.try_recv() {
            Ok(m) => min_accel = m,
            TryRecvError => (),
        }

        // A minimum acceleration
        let mut new_accel = match accels.back() {
            Some(a) => a.clone(),
            None => Vec3{x: 0.0, y: 0.0, z: 0.0},
        };
        if accel.x > min_accel {
            new_accel.x = accel.x;
        }
        if accel.y > min_accel {
            new_accel.y = accel.y;
        }
        if accel.z > min_accel {
            new_accel.z = accel.z;
        }

        if accel.x > min_accel || accel.y > min_accel || accel.z > min_accel{
            accels.push_back(new_accel);
            if accels.len() >= TOTAL{
                accels.pop_front();
            }
        }
        *dj.lock().unwrap() = movement::extract_jerk(&accels);
    }
}

fn main() {
    let (display, mut events_loop, text) = render::init();
    
    let mut patterns = create_colour_clouds();

    let mut socket = UdpSocket::bind("0.0.0.0:44444").unwrap();

    let mut port = arduino::open();

    let (sender_min, min_recv) = mpsc::channel();

    let (sender_jerk, recv) = mpsc::channel();

    let average_jerk = Arc::new( Mutex::new(0.0) );
    
    let dj = Arc::new( Mutex::new(Vec3{x:0.0, y:0.0, z:0.0}) );

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

    const J_BUFF_LEN: usize = 100;
    let mut jerks = Buffer::new_fill(1.0, J_BUFF_LEN);

    let min_accel = pre_balance(&mut buf, &mut socket);
    let mut min_accel2 = min_accel;

    let dj2 = dj.clone();
    std::thread::spawn(move ||{
        gather_accels(socket, dj2, min_accel, min_recv);
    });

    let mut amp = 16.0;
    let mut last_jerk = Vec3{x: 0.0, y: 0.0, z: 0.0};
    loop {

        let mut dj_total = dj.lock().unwrap().clone();

        //amp here
        dj_total.scale(amp);

        if dj_total != last_jerk {
            last_jerk = dj_total;
            dj_total = movement::clamp_jerk(&dj_total);
            dj_total = Vec3{x: (dj_total.x + 1.0).log(2.0), 
                y: (dj_total.y + 1.0).log(2.0), 
                z: (dj_total.z + 1.0).log(2.0)};

            let jerk = max( dj_total.x, max(dj_total.y, dj_total.z) );
            jerks.add(jerk);
        }

        /*
        let last_jerk = jerks.last();
        match last_jerk {
            Some(j) if j != jerk => jerks.add(jerk),
            Some(_) => (),
            None => jerks.add(jerk),
        };
        */

        let colour = buffer::average(&jerks);
        *average_jerk.lock().unwrap() = colour;

        let (output1, output2) = render::to_text(colour, &dj_total, jerks.cap(), &amp, &min_accel2);

        let text_out1 = glium_text::TextDisplay::new(&text.system, &*text.font, &output1[..]);
        let text_out2 = glium_text::TextDisplay::new(&text.system, &*text.font, &output2[..]);

        let (matrix1, matrix2) = get_matrix();

        let mut target = display.draw();
        target.clear_color(colour, colour, colour, 1.0);
        glium_text::draw(&text_out1, &text.system, &mut target, matrix1, text.color).unwrap();
        glium_text::draw(&text_out2, &text.system, &mut target, matrix2, text.color).unwrap();
        target.finish().unwrap();

        if render::events(&mut events_loop, &mut jerks, &mut amp, &mut min_accel2, &sender_min) {
            break;
        };
        count += 1;
    }
}

fn get_matrix() -> ([[f32;4];4], [[f32;4];4]){
    let matrix1 = [
        [0.05, 0.0, 0.0, 0.0],
        [0.0, 0.08, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-1.0, 0.8, 0.0, 1.0]
    ];

        let matrix2 = [
            [0.05, 0.0, 0.0, 0.0],
            [0.0, 0.08, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-1.0, 0.7, 0.0, 1.0]
        ];
            (matrix1, matrix2)
}
