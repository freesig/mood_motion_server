use glium::{glutin, Surface, Display};
use std;
use std::ops::AddAssign;
use ::buffer::Buffer;

#[derive(Clone, Serialize, Deserialize)]
pub struct Colour<T>{
    pub r: T,
    pub g: T,
    pub b: T,
}

impl Colour<i16>{
    pub fn scale(&mut self, val: f32){
        self.r = (self.r as f32 * val) as i16;
        self.g = (self.g as f32 * val) as i16;
        self.b = (self.b as f32 * val) as i16;
    }
}

impl AddAssign for Colour<i16>{
    fn add_assign(&mut self, other: Colour<i16>) {
        *self = Colour{
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

impl std::fmt::Display for Colour<i16>{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{},{},", self.r, self.g, self.b)
    }
}

pub fn init() -> (Display, glutin::EventsLoop){

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let dispay = Display::new(window, context, &events_loop).unwrap();
    (dispay, events_loop)

}

pub fn events(events_loop: &mut glutin::EventsLoop, jerks: &mut Buffer) -> bool {
    use glutin::{WindowEvent, KeyboardInput, VirtualKeyCode, ElementState};
    let mut close = false;
    events_loop.poll_events(|event| {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                WindowEvent::Closed => close = true,
                WindowEvent::KeyboardInput{ input, .. } => {
                    match input{
                        KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Minus),
                        state: ElementState::Pressed, .. } => {
                            println!("minus");
                            println!( "Buffer size: {}", jerks.cap() );
                            let mut new_size = jerks.cap() / 2; 
                            if new_size < 4 {
                                new_size = 4;
                            }
                            println!("new size: {}", new_size);
                            ::buffer::new_buff_size(new_size, jerks);
                            println!( "Buffer size: {}", jerks.cap() );
                        }, 
                        KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::Equals),
                        state: ElementState::Pressed, ..} => {
                            println!("equals");
                            println!( "Buffer size: {}", jerks.cap() );
                            let new_size = jerks.cap() * 2; 
                            println!("new size: {}", new_size);
                            ::buffer::new_buff_size(new_size, jerks);
                            println!( "Buffer size: {}", jerks.cap() );
                        }, 
                        _ => (),
                    };
                },
                _ => ()
            },
            _ => (),
        }
    });
    close
}
