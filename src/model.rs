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

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        tex_coords: [0.4131759, 0.99240386],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.0048659444, 0.56958647],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.28081453, 0.05060294],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        tex_coords: [0.85967, 0.1526709],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [0.9414737, 0.7347359],
    }, // E
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
                tex_coords: [rand::random::<f32>(), rand::random::<f32>()],
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
