extern crate glium;

use glium::glutin;
use glium::Surface;
use glium::glutin::event::VirtualKeyCode;
use glium::backend::glutin::glutin::dpi::PhysicalPosition;


// fn input_handler(input: glutin::event::KeyboardInput) {
fn input_handler() {
    println!("Input!");
}

fn update_cursor_pos(cursor_x: f64, cursor_y: f64) {
    println!("Mouse: {}, {}", cursor_x, cursor_y);
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now()
            + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput {
                    input:
                        glutin::event::KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                     ..
                } => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput {..} => input_handler(),
                glutin::event::WindowEvent::CursorMoved {
                    position: PhysicalPosition { x, y }, ..
                } => update_cursor_pos(x,y),
                _ => (),
            },
            _ => (),
        }
    });

}
