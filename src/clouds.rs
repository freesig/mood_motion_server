use std;
use std::fs::File;
use std::path::Path;
use std::vec::Vec;
use std::vec::IntoIter;
use std::iter::Cycle;
use std::io::Read;
use ::render::Colour;
use serde_json;
use serde_json::Value;

pub struct Cloud{
    pattern: Cycle< std::vec::IntoIter<u8> >,
    min: u8,
    max: u8,
}

pub enum Channel{
    One, 
    Two,
}

pub struct CloudSet<'a>{
    pub p: &'a Path,
    pub mood: i16,
    pub channel: Channel,
}

pub struct ColourCloud{
    pattern: Cycle< std::vec::IntoIter< Colour<u8> > >,
    mood: i16,
    channel: Channel,
}

impl Iterator for Cloud{
    type Item = u8;
    fn next(&mut self) -> Option<u8>{
        self.pattern.next()
    }
}

impl Iterator for ColourCloud{
    type Item = Colour<u8>;
    fn next(&mut self) -> Option< Colour<u8> >{
        self.pattern.next()
    }
}

pub fn load_json(cs: CloudSet) -> Option<ColourCloud> {
    match File::open(&cs.p){
        Ok(mut f) => {
            match serde_json::from_reader(f){
                Ok(v) =>{
                    let mut pattern: Vec< Colour<u8> > = Vec::new();
                    let val_arr = match v{
                        Value::Array(data) => Some(data),
                        _ => None,
                    };
                    match val_arr{
                        Some(v_arr) =>{
                            for d in v_arr{
                                pattern.push( serde_json::from_value(d).unwrap() );
                            }
                            let size = pattern.len();
                            let it = pattern.into_iter();
                            let mut c = ColourCloud{pattern: it.cycle(), 
                                mood: cs.mood, channel: cs.channel };
                            println!("Read in {} colours to cloud", size);
                            Some(c)
                        },
                        None => None,
                    }
                },
                Err(_) => {
                    println!("Couldn't load json");
                    None
                },
            }
        },
        Err(_) => {
            println!("Could not load file into memory");
            None
        },
    }
}

pub fn load(p: &Path, min: u8, max: u8) -> Option<Cloud> {
    match File::open(&p){
        Ok(mut f) => {
            let mut pattern: Vec<u8> = Vec::new();
            let mut contents = String::new();
            match f.read_to_string(&mut contents){
                Ok(size) => {
                    let mut num = String::new();
                    for c in contents.chars(){
                        if c.is_digit(10) {
                            num.push(c);
                        }else{
                            pattern.push( num.parse::<u8>().unwrap() );
                            num = String::new();
                        }
                    }
                    let it = pattern.into_iter();
                    let mut c = Cloud{pattern: it.cycle(), min, max};
                    println!("Read in {} bytes to cloud", size);
                    Some(c)
                },
                Err(_) => {
                    println!("Could not load file into memory");
                    None
                },
            }
        },
        Err(why) => {
            println!("Couldn't load file because: {}", why);
            None
        },
    }
}

pub fn cloud_to_light(c: &mut Cloud) -> Colour<i16>{
    let brightness = c.next().unwrap();
    let brightness = (brightness as f32 / (c.max - c.min) as f32 * 255.0) as i16;
    Colour{r: brightness, g: brightness, b: brightness}
}

pub fn cloud_to_colour(c: &mut ColourCloud, jerk: f32) -> Colour<i16>{
    let clamp = 5;
    let jerk = (255.0 * jerk) as i16;
    let mut distance = (c.mood - jerk).abs();
    distance /= clamp;
    let distance = std::cmp::max(distance, 1) as f32;
    let brightness = 255.0 / distance; 
    let brightness = brightness / 255.0;
    println!("distance: {}", distance);
    println!("brightness: {}", brightness);
    let c = c.next().unwrap();
    let mut c = Colour{r: c.r as i16, g: c.g as i16, b: c.b as i16};
    c.scale(brightness);
    c
}
