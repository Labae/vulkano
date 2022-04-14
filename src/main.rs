use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder, Window},
    dpi::LogicalSize};
use anyhow::Result;

fn main() {
     // to use a visually nice colored log.
    pretty_env_logger::init();

    let width: i32 = 1024;
    let height: i32 = 768;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
            .with_title("Vulkan Tutorial (Rust)")
            .with_inner_size(LogicalSize::new(width, height))
            .build(&event_loop).unwrap();

    let mut app = App::create(&window).unwrap();
    let mut destroying = false;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared if !destroying => app.render(&window).unwrap(),
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                app.destroy();
            }
            _ => {}
        }
    });
}

struct App {}

impl App {
    fn create(window: &Window) -> Result<Self> {
        Ok(Self {})
    }

    fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    fn destroy(&mut self) {}
}