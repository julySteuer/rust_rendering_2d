use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use models::{Rectangle::Rect, World::World};

fn main() {
    let WIDTH:u32 = 600;
    let HEIGHT:u32 = 600;
    let size = LogicalSize::new(WIDTH, HEIGHT);
    let event_loop = EventLoop::new();
    let world = World::new(&WIDTH, &HEIGHT);
    let window = WindowBuilder::new().with_inner_size(size).with_max_inner_size(size).build(&event_loop).unwrap();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };
    let rect:Rect = Rect::new(300, 400, 400, 500, WIDTH as usize,HEIGHT as usize);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::RedrawRequested(_) = event{ 
            rect.draw(pixels.get_frame());
            pixels.render().unwrap();
        }
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
        window.request_redraw();
    });
}  