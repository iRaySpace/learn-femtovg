use femtovg::{renderer::OpenGl, Canvas, Color};
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};

fn main() {
    let window_builder = WindowBuilder::new().with_title("Learn Femtovg");
    let event_loop = EventLoop::new();
    let windowed_context = unsafe {
        ContextBuilder::new()
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap()
    };
    let renderer = unsafe {
        OpenGl::new_from_function(|s| windowed_context.get_proc_address(s) as *const _)
            .expect("Cannot create renderer")
    };
    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

    event_loop.run(move |event, _, control_flow| {
        let window = windowed_context.window();
        *control_flow = ControlFlow::Poll;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(*physical_size);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let size = window.inner_size();
                let dpi_factor = window.scale_factor();
                
                canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
                canvas.clear_rect(
                    0,
                    0,
                    size.width as u32,
                    size.height as u32,
                    Color::rgbf(0.0, 0.0, 1.0),
                );
                canvas.flush();

                windowed_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    })
}
