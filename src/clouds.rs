use std;
use std::fs::File;
use std::path::Path;
use std::vec::Vec;
use std::vec::IntoIter;
use std::iter::Cycle;
use std::io::Read;

pub struct Cloud{
    pattern: Cycle< std::vec::IntoIter<u8> >,
    min: u8,
    max: u8,
}

impl Iterator for Cloud{
    type Item = u8;
    fn next(&mut self) -> Option<u8>{
        self.pattern.next()
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

pub fn cloud_to_light(c: &mut Cloud) -> ::render::Colour<i16>{
    let brightness = c.next().unwrap();
    let brightness = (brightness as f32 / (c.max - c.min) as f32 * 255.0) as i16;
    ::render::Colour{r: brightness, g: brightness, b: brightness}
}
