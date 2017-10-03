use glium::{glutin, Surface, Display};

pub fn init() -> (Display, glutin::EventsLoop){

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let dispay = Display::new(window, context, &events_loop).unwrap();
    (dispay, events_loop)

}

pub fn events(events_loop: & mut glutin::EventsLoop) -> bool {
    let mut close = false;
    events_loop.poll_events(|event| {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => close = true,
                _ => ()
            },
            _ => (),
        }
    });
    close
}
