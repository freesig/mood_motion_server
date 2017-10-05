#[macro_use]
extern crate glium;

use std::net::UdpSocket;
use std::collections::VecDeque;
use glium::{glutin, Surface};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

mod arduino;
mod render;
mod movement;

use arduino::Port;

use movement::Vec3;
use movement::max;

const TOTAL: usize = 50;


fn main() {
    let (display, mut events_loop) = render::init();

    let mut socket = UdpSocket::bind("0.0.0.0:44444").unwrap();

    let mut port = arduino::open();

    let (sender_jerk, recv) = mpsc::channel();

    std::thread::spawn(move ||{
        match port {
            Port::Open(mut p) => arduino::interact(&mut p, recv).unwrap(),
            Port::Dummy =>(),
        };
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

        let mut target = display.draw();
        target.clear_color(colour, colour, colour, 1.0);
        target.finish().unwrap();

        if count >= 50 {
            let msg = movement::jerk_to_light(colour, 0);
            sender_jerk.send(msg);
            let msg = movement::jerk_to_light(colour, 1);
            sender_jerk.send(msg);
            count = 0;
        }


        if render::events(&mut events_loop) {
            break;
        };
        count += 1;
    }
}
