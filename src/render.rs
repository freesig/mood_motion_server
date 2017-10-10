use glium::{glutin, Surface, Display};
use std;
use std::ops::AddAssign;
use ::buffer::Buffer;
use ::font::Text;
use movement::Vec3;
use std::sync::mpsc::Sender;

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

pub fn init() -> (Display, glutin::EventsLoop, Text){

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = Display::new(window, context, &events_loop).unwrap();
    let t = Text::new(&display);
    (display, events_loop, t)

}

pub fn events(events_loop: &mut glutin::EventsLoop, jerks: &mut Buffer, 
              amp: &mut f32, min_accel: &mut f32, min_sender: &Sender<f32>) -> bool {
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
                        KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::W),
                        state: ElementState::Pressed, ..} => {
                            *amp *= 2.0;
                        },
                        KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::S),
                        state: ElementState::Pressed, ..} => {
                            *amp /= 2.0;
                            if *amp <= 1.0 {
                                *amp = 1.0;
                            }
                        },
                        KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::D),
                        state: ElementState::Pressed, ..} => {
                            *amp += 1.0;
                        },
                        KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::A),
                        state: ElementState::Pressed, ..} => {
                            *amp -= 1.0;
                            if *amp <= 1.0 {
                                *amp = 1.0;
                            }
                        },
                        KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::X),
                        state: ElementState::Pressed, ..} => {
                            *min_accel += 0.5;
                            min_sender.send(*min_accel);
                        },
                        KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::Z),
                        state: ElementState::Pressed, ..} => {
                            *min_accel -= 0.5;
                            if *min_accel <= 0.0 {
                                *min_accel = 0.0;
                            }
                            min_sender.send(*min_accel);
                        },
                        KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::Q),
                        state: ElementState::Pressed, ..} => {
                            close = true;
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

pub fn to_text(avg_jerk: f32, cur_jerk: &Vec3,
               buf_size: usize, amp: &f32, min_accel: &f32) -> (String, String){
    let out1 = format!("Average jerk: {} current jerk {}", avg_jerk, cur_jerk);
    let out2 = format!("num jerks: {} amp: {} min acc: {}", buf_size, amp, min_accel);
    (out1, out2)
}
