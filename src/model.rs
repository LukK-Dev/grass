use anyhow::anyhow;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Zeroable, Pod, Clone, Copy, Debug)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: &[Vertex], indices: &[u32]) -> Self {
        Self {
            vertices: Vec::from(vertices),
            indices: Vec::from(indices),
        }
    }

    pub fn create_circle(resolution: usize) -> anyhow::Result<Mesh> {
        if resolution < 3 {
            return Err(anyhow!(
                "cannot create circle-mesh from a resolution less than three"
            ));
        }

        let mut vertices = vec![];
        let mut indices = vec![];

        let step = std::f32::consts::TAU / resolution as f32;
        for i in 0..resolution {
            let x = (step * i as f32).cos();
            let y = (step * i as f32).sin();
            vertices.push(Vertex {
                position: [x, y, 0.0],
                tex_coords: [x * 0.5 + 0.5, 1.0 - (y * 0.5 + 0.5)],
            })
        }

        let num_triangles = resolution - 2;
        for i in 0..num_triangles {
            indices.push(0);
            indices.push(i as u32 + 1);
            indices.push(i as u32 + 2);
        }

        Ok(Self { vertices, indices })
    }

    pub fn create_rectangle() -> Self {
        Self {
            vertices: vec![
                Vertex {
                    position: [-1.0, 1.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, -1.0, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [1.0, -1.0, 0.0],
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    position: [1.0, 1.0, 0.0],
                    tex_coords: [1.0, 0.0],
                },
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
        }
    }
}
