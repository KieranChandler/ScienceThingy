extern crate glium;

use glium::glutin;
use glium::glutin::event::VirtualKeyCode;
use glium::glutin::event::ElementState;
use glium::glutin::event::MouseButton;
use glium::backend::glutin::glutin::dpi::PhysicalPosition;
use glium::Surface;
use glium::implement_vertex;

fn input_handler() {
    println!("Input!");
}

fn update_cursor_pos(cursor_x: f64, cursor_y: f64) {
    //println!("Mouse: {}, {}", cursor_x, cursor_y);
}

fn left_click(cursor_pos: (f64, f64)) {
    let (cursor_x, cursor_y) = cursor_pos;
    println!("Mouseclick @: {}, {}", cursor_x, cursor_y);
}

fn main() {
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let mut cursor_pos: (f64, f64) = (0.0, 0.0);

    // code for drawing shader
    let vertex_shader_src = r#"
        #version 140
    
        in vec2 position;
    
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
    
    let program = glium::Program::from_source(
        &display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
    // code for drawing triangle
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.0, 0.5] };
    let vertex3 = Vertex { position: [0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    

    event_loop.run(move |ev, _, control_flow| {
        let mut frame = display.draw(); 
        //let (mut frame, uniforms) = common::begin_frame(&display);
        frame.clear_color(0.0, 0.0, 1.0, 1.0);
        frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                   &Default::default()).unwrap();
        frame.finish().unwrap(); 
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
