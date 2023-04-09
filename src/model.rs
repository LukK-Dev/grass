use anyhow::anyhow;
use bytemuck::{Pod, Zeroable};
use log::debug;

#[repr(C)]
#[derive(Zeroable, Pod, Clone, Copy, Debug)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
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
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

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
            vertices.push(Vertex {
                position: [(step * i as f32).cos(), (step * i as f32).sin(), 0.0],
                color: [
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                ],
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
}
