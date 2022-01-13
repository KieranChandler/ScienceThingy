extern crate glium;

use glium::{Display, Frame, implement_vertex, Program, Surface, VertexBuffer};
use glium::index::NoIndices;


#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct Shape {
    vertex_buffer: VertexBuffer<Vertex>,
    indices: NoIndices,
}

impl<'a> From<&'a Shape> for glium::vertex::VerticesSource<'a> {
    fn from(shape: &'a Shape) -> glium::vertex::VerticesSource<'a> {
        (&shape.vertex_buffer).into()
    }
}

impl<'a> Into<glium::index::IndicesSource<'a>> for &'a Shape {
    fn into(self) -> glium::index::IndicesSource<'a> {
        glium::index::IndicesSource::NoIndices {
            primitives: glium::index::PrimitiveType::TrianglesList,
        }
    }
}

pub fn generate(display: &Display) -> Shape {
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.0, 0.5] };
    let vertex3 = Vertex { position: [0.5, -0.25] };
    let vertices = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    return Shape { vertex_buffer, indices };
}

pub fn draw(frame: &mut Frame, shape: &Shape, program: &Program) {
    frame.draw(
        &shape.vertex_buffer, &shape.indices, program,
        &glium::uniforms::EmptyUniforms,
        &Default::default(),
    ).unwrap()
}