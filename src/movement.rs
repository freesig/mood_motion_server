use std;
use std::net::UdpSocket;
use std::collections::VecDeque;
use std::ops::{Sub, AddAssign, DivAssign};

#[derive(Debug, Clone)]
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
    pub fn scale(&mut self, amount: f32) {
        *self = Vec3{
            x: self.x * amount,
            y: self.y * amount,
            z: self.z * amount,
        };
    }
}

pub fn max(l: f32, r: f32) -> f32{
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

pub fn read(mut buf: & mut [u8], socket: & mut UdpSocket) -> Vec3 {
    let (amt, src) = socket.recv_from(&mut buf).unwrap();

    let data = std::str::from_utf8(&buf[..]).unwrap();
    let mut peices = data.split(",");
    let x: f32 = peices.next().unwrap().parse().unwrap();
    let y: f32 = peices.next().unwrap().parse().unwrap();
    let z: f32 = peices.next().unwrap().parse().unwrap();
    Vec3{x, y, z}
}

pub fn extract_jerk(accels: & VecDeque<Vec3>) -> Vec3 {
    let mut dj_total = Vec3{ x: 0.0, y: 0.0, z: 0.0 };
    let mut a_iter = accels.iter();
    loop {
        match a_iter.next() {
            Some(a_cur) => {
                match a_iter.next() {
                    Some(a_next) => dj_total += a_next - a_cur,
                    None => break,
                }
            },
            None => break,
        }
    }

    dj_total.scale(1.0 / accels.len() as f32);
    dj_total
}

pub fn jerk_to_light(colour: f32, light: i16) -> String{
    let colour = (255.0 * colour) as i32;
    format!( "l{}{},{},{},x", light, colour, colour, colour)
}

pub fn clamp_jerk(j: & Vec3) -> Vec3{
    let mut jc = Vec3{x: 0.0, y: 0.0, z: 0.0};
    jc.x = max( min( j.x.abs(), 1.0 ), 0.0);
    jc.y = max( min( j.y.abs(), 1.0 ), 0.0);
    jc.z = max( min( j.z.abs(), 1.0 ), 0.0);
    jc
}

