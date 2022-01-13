extern crate glium;

use glium::Display;
use crate::shape::{Shape, Vertex};


pub fn generate(display: &Display) -> Shape {
    let vertex1 = Vertex { position: [-0.5, -0.5, 0.0] };
    let vertex2 = Vertex { position: [0.0, 0.5, 0.0] };
    let vertex3 = Vertex { position: [0.5, -0.25, 0.0] };
    let vertices = vec![vertex1, vertex2, vertex3];

    let indices = vec![0, 1, 2];

    let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &indices).unwrap();

    return Shape { vertex_buffer, index_buffer };
}
