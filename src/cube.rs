use glium;

use std::f32;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
    normal: (f32, f32, f32),
}

implement_vertex!(Vertex, position, normal);

pub struct Cube {
    positions: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}

impl Cube {
    pub fn new<F>(facade: &F) -> Cube
    where
        F: glium::backend::Facade,
    {
        let verticies = vec![
            Vertex {
                position: (-1., 1., -1.),
                normal: (-1., 0., 0.),
            },
            Vertex {
                position: (-1., -1., -1.),
                normal: (-1., 0., 0.),
            },
            Vertex {
                position: (-1., -1., 1.),
                normal: (-1., 0., 0.),
            },
            Vertex {
                position: (-1., 1., 1.),
                normal: (-1., 0., 0.),
            },
            Vertex {
                position: (-1., 1., -1.),
                normal: (-1., 0., 0.),
            },
            Vertex {
                position: (-1., -1., 1.),
                normal: (-1., 0., 0.),
            },
            Vertex {
                position: (1., -1., -1.),
                normal: (1., 0., 0.),
            },
            Vertex {
                position: (1., 1., -1.),
                normal: (1., 0., 0.),
            },
            Vertex {
                position: (1., -1., 1.),
                normal: (1., 0., 0.),
            },
            Vertex {
                position: (1., 1., -1.),
                normal: (1., 0., 0.),
            },
            Vertex {
                position: (1., 1., 1.),
                normal: (1., 0., 0.),
            },
            Vertex {
                position: (1., -1., 1.),
                normal: (1., 0., 0.),
            },
            Vertex {
                position: (1., -1., -1.),
                normal: (0., 0., -1.),
            },
            Vertex {
                position: (-1., -1., -1.),
                normal: (0., 0., -1.),
            },
            Vertex {
                position: (-1., 1., -1.),
                normal: (0., 0., -1.),
            },
            Vertex {
                position: (1., 1., -1.),
                normal: (0., 0., -1.),
            },
            Vertex {
                position: (1., -1., -1.),
                normal: (0., 0., -1.),
            },
            Vertex {
                position: (-1., 1., -1.),
                normal: (0., 0., -1.),
            },
            Vertex {
                position: (-1., -1., 1.),
                normal: (0., 0., 1.),
            },
            Vertex {
                position: (1., -1., 1.),
                normal: (0., 0., 1.),
            },
            Vertex {
                position: (-1., 1., 1.),
                normal: (0., 0., 1.),
            },
            Vertex {
                position: (1., -1., 1.),
                normal: (0., 0., 1.),
            },
            Vertex {
                position: (1., 1., 1.),
                normal: (0., 0., 1.),
            },
            Vertex {
                position: (-1., 1., 1.),
                normal: (0., 0., 1.),
            },
            Vertex {
                position: (-1., -1., 1.),
                normal: (0., -1., 0.),
            },
            Vertex {
                position: (-1., -1., -1.),
                normal: (0., -1., 0.),
            },
            Vertex {
                position: (1., -1., -1.),
                normal: (0., -1., 0.),
            },
            Vertex {
                position: (1., -1., 1.),
                normal: (0., -1., 0.),
            },
            Vertex {
                position: (-1., -1., 1.),
                normal: (0., -1., 0.),
            },
            Vertex {
                position: (1., -1., -1.),
                normal: (0., -1., 0.),
            },
            Vertex {
                position: (-1., 1., -1.),
                normal: (0., 1., 0.),
            },
            Vertex {
                position: (-1., 1., 1.),
                normal: (0., 1., 0.),
            },
            Vertex {
                position: (1., 1., -1.),
                normal: (0., 1., 0.),
            },
            Vertex {
                position: (-1., 1., 1.),
                normal: (0., 1., 0.),
            },
            Vertex {
                position: (1., 1., 1.),
                normal: (0., 1., 0.),
            },
            Vertex {
                position: (1., 1., -1.),
                normal: (0., 1., 0.),
            },
        ];

        Cube {
            positions: glium::VertexBuffer::new(facade, &verticies).unwrap(),
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        }
    }

    pub fn get_positions(&self) -> &glium::VertexBuffer<Vertex> {
        &self.positions
    }
    pub fn get_indices(&self) -> &glium::index::NoIndices {
        &self.indices
    }
}
