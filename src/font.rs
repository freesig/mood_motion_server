use std;
use glium_text;
use glium::Display;
use cgmath;
use glium_text::{TextDisplay, TextSystem};

pub struct Text {
    pub font: Box<glium_text::FontTexture>,
    pub system: TextSystem,
    //pub matrix: cgmath::Matrix4<f32>,
    pub color: (f32, f32, f32, f32),
}

impl Text{
    pub fn new(display: &Display) -> Self{
        let system = glium_text::TextSystem::new(display);
        let font = glium_text::FontTexture::new(
            display, &include_bytes!("../font/Verdana.ttf")[..], 18, 
            glium_text::FontTexture::ascii_character_list()).unwrap();
        Text{font: Box::new(font), system, color: (0.0, 0.0, 0.0, 1.0)}
    }
}
