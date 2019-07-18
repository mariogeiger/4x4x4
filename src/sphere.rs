use glium;

use std::f32;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
}

//implement_vertex!(Vertex, position);

impl glium::vertex::Vertex for Vertex {
    #[inline]
    fn build_bindings() -> glium::vertex::VertexFormat {
        use std::borrow::Cow;

        // For the sphere the vertex positions and the normals are the same
        Cow::Owned(vec![
            (
                Cow::Borrowed("position"),
                0,
                glium::vertex::AttributeType::F32F32F32,
                false,
            ),
            (
                Cow::Borrowed("normal"),
                0,
                glium::vertex::AttributeType::F32F32F32,
                false,
            ),
        ])
    }
}

pub struct Sphere {
    positions: glium::VertexBuffer<Vertex>,
    indices: glium::IndexBuffer<u16>,
}

impl Sphere {
    pub fn new<F>(facade: &F, slices: u16, stacks: u16) -> Sphere
    where
        F: glium::backend::Facade,
    {
        let size = 2 + slices * (stacks - 1);
        let mut positions = Vec::with_capacity(3 * size as usize);

        let alpha = f32::consts::PI / stacks as f32;
        let beta = 2.0 * f32::consts::PI / slices as f32;

        positions.push(Vertex {
            position: (0.0, 0.0, 1.0),
        });

        for i in 1..stacks {
            let i = i as f32;
            for j in 0..slices {
                let j = j as f32;
                let r = (i * alpha).sin();
                let z = (i * alpha).cos();
                let y = (j * beta).sin() * r;
                let x = (j * beta).cos() * r;

                positions.push(Vertex {
                    position: (x, y, z),
                });
            }
        }

        positions.push(Vertex {
            position: (0.0, 0.0, -1.0),
        });

        let mut indices: Vec<u16> =
            Vec::with_capacity((3 * slices * (2 + 2 * (stacks - 2))) as usize);

        for i in 1..slices {
            indices.push(0);
            indices.push(i);
            indices.push(i + 1);
        }
        indices.push(0);
        indices.push(slices);
        indices.push(1);

        for j in 1..stacks - 1 {
            for i in 1..slices {
                indices.push(1 + (j - 1) * slices + i);
                indices.push(0 + (j - 1) * slices + i);
                indices.push(0 + j * slices + i);

                indices.push(0 + j * slices + i);
                indices.push(1 + j * slices + i);
                indices.push(1 + (j - 1) * slices + i);
            }

            indices.push(1 + (j - 1) * slices);
            indices.push(slices + (j - 1) * slices);
            indices.push(slices + j * slices);

            indices.push(slices + j * slices);
            indices.push(1 + j * slices);
            indices.push(1 + (j - 1) * slices);
        }

        for i in 1..slices {
            indices.push(size - 1);
            indices.push(size - i - 1);
            indices.push(size - i - 2);
        }
        indices.push(size - 1);
        indices.push(size - slices - 1);
        indices.push(size - 2);

        Sphere {
            positions: glium::VertexBuffer::new(facade, &positions).unwrap(),
            indices: glium::IndexBuffer::new(
                facade,
                glium::index::PrimitiveType::TrianglesList,
                &indices,
            )
            .unwrap(),
        }
    }

    pub fn get_positions(&self) -> &glium::VertexBuffer<Vertex> {
        &self.positions
    }
    pub fn get_indices(&self) -> &glium::IndexBuffer<u16> {
        &self.indices
    }
}
