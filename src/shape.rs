extern crate glium;

use glium::{Frame, implement_vertex, IndexBuffer, Program, Surface, VertexBuffer};


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f64; 3],
}

implement_vertex!(Vertex, position);

pub struct Shape {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u32>,
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

pub fn draw(frame: &mut Frame, shape: &Shape, program: &Program) {
    frame.draw(
        &shape.vertex_buffer, &shape.index_buffer, program,
        &glium::uniforms::EmptyUniforms,
        &Default::default(),
    ).unwrap()
}