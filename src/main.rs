extern crate glium;

pub mod unlit_material;
pub mod triangle;
pub mod shape;
pub mod icosphere;

use glium::glutin;
use glium::glutin::event::VirtualKeyCode;
use glium::glutin::event::ElementState;
use glium::glutin::event::MouseButton;
use glium::backend::glutin::glutin::dpi::PhysicalPosition;
use glium::Surface;

fn input_handler() {
    println!("Input!");
}

fn left_click(cursor_pos: (f64, f64)) {
    let (cursor_x, cursor_y) = cursor_pos;
    println!("Mouseclick @: {}, {}", cursor_x, cursor_y);
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let mut cursor_pos: (f64, f64) = (0.0, 0.0);

    let shape = icosphere::generate(&display);
    let program = unlit_material::generate_unlit_material(&display);

    event_loop.run(move |ev, _, control_flow| {
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);

        shape::draw(&mut frame, &shape, &program);

        frame.finish().unwrap();

        let next_frame_time = std::time::Instant::now()
            + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
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
                }
                glutin::event::WindowEvent::KeyboardInput { .. } => input_handler(),
                glutin::event::WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                    ..
                } => left_click(cursor_pos),
                glutin::event::WindowEvent::CursorMoved {
                    position: PhysicalPosition { x, y }, ..
                } => (cursor_pos = (x, y)),
                _ => (),
            },
            _ => (),
        }
    });
}
