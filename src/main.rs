#[macro_use]
extern crate glium;

use std::net::UdpSocket;
use std::ops::{Sub, AddAssign, DivAssign};
use glium::{glutin, Surface};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

mod arduino;

use arduino::Port;

const TOTAL: i32 = 50;
const MIN_ACCEL: f32 = 0.5;

#[derive(Debug)]
struct Vec3{
    x: f32,
    y: f32,
    z: f32,
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3{
    type Output = Vec3;

    fn sub(self, other: &'b Vec3) -> Vec3 {
        Vec3{ x: self.x - other.x,
        y: self.y - other.y,
        z: self.z - other.z
        }
    }
}

impl AddAssign for Vec3{
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Vec3{
    fn scale(&mut self, amount: f32) {
        *self = Vec3{
            x: self.x * amount,
            y: self.y * amount,
            z: self.z * amount,
        };
    }
}

fn max(l: f32, r: f32) -> f32{
    if l.ge(&r) {
        l
    }else{
        r
    }
}

fn min(l: f32, r: f32) -> f32{
    if l.le(&r) {
        l
    }else{
        r
    }
}

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut socket = UdpSocket::bind("0.0.0.0:44444").unwrap();

    let mut closed = false;
    // read from the socket
    let mut buf = [0; 100];
    let mut history: Vec<Vec3> = Vec::new();

    let mut port = arduino::open();

    let (sender_jerk, recv) = mpsc::channel();

    std::thread::spawn(move ||{
        match port {
            Port::Open(mut p) => arduino::interact(&mut p, recv).unwrap(),
            Port::Dummy =>(),
        };
    });

    let mut count = 0;

    while !closed {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        let data = std::str::from_utf8(&buf[..]).unwrap();
        let mut peices = data.split(",");
        let x: f32 = peices.next().unwrap().parse().unwrap();
        let y: f32 = peices.next().unwrap().parse().unwrap();
        let z: f32 = peices.next().unwrap().parse().unwrap();
        let accel = Vec3{x, y, z};

        // A minimum acceleration
        if accel.x > MIN_ACCEL || accel.y > MIN_ACCEL || accel.z > MIN_ACCEL {


            history.push(accel);
            if history.len() >= TOTAL as usize{
                history.remove(0);
            }

            let mut dj_total = Vec3{ x: 0.0, y: 0.0, z: 0.0 };
            for i in 0..(history.len() - 1) {
                dj_total += &history[i+1] - &history[i];

            }
            dj_total.scale(1.0 / history.len() as f32);
            //println!("dj_total: {:?}", dj_total);

            dj_total.x = max( min( dj_total.x.abs(), 1.0 ), 0.0);
            dj_total.y = max( min( dj_total.y.abs(), 1.0 ), 0.0);
            dj_total.z = max( min( dj_total.z.abs(), 1.0 ), 0.0);
            let mut target = display.draw();
            target.clear_color(dj_total.x.abs(), dj_total.y.abs(), dj_total.z.abs(), 1.0);
            target.finish().unwrap();

            if count >= 100 {
                let mut amount = dj_total.x.abs() + dj_total.y.abs() + dj_total.z.abs();
                amount /= 3.0;

                let amount = (255.0 * amount) as i16;
                let amount = format!( "l{}", amount.to_string() );
                sender_jerk.send(amount);
                count = 0;
            }

        }

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => ()
                },
                _ => (),
            }
        });

        count += 1;
    }
}
