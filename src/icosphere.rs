extern crate glium;

use glium::Display;
use crate::shape::{Shape, Vertex};

pub fn generate(display: &Display) -> Shape {

    let x = 0.525731112119133606;
    let z = 0.850650808352039932;
    let vertices = vec![
        Vertex { position: [-x, 0.0, z] },
        Vertex { position: [x, 0.0, z] },
        Vertex { position: [-x, 0.0, -z] },
        Vertex { position: [x, 0.0, -z] },
        Vertex { position: [0.0, z, x] },
        Vertex { position: [0.0, z, -x] },
        Vertex { position: [0.0, -z, x] },
        Vertex { position: [0.0, -z, -x] },
        Vertex { position: [z, x, 0.0] },
        Vertex { position: [-z, x, 0.0] },
        Vertex { position: [z, -x, 0.0] },
        Vertex { position: [-z, -x, 0.0] },
    ];
    let indices = [
        0, 4, 1,
        0, 9, 4,
        9, 5, 4,
        4, 5, 8,
        4, 8, 1,
        8, 10, 1,
        8, 3, 10,
        5, 3, 8,
        5, 2, 3,
        2, 7, 3,
        7, 10, 3,
        7, 6, 10,
        7, 11, 6,
        11, 0, 6,
        0, 1, 6,
        6, 1, 10,
        9, 0, 11,
        9, 11, 2,
        9, 2, 5,
        7, 2, 11,
    ];

    // // generate normals
    // let d1: [f32; 3];
    // let d2: [f32; 3];
    // let norm: [f32; 3];
    // 
    // for i in 0..20 {
    //     for j in 0..3 {
    //         d1[j] =
    //     }
    // }

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(&display,
                                               glium::index::PrimitiveType::TrianglesList,
                                               &indices).unwrap();
    return Shape { vertex_buffer, index_buffer };
}
