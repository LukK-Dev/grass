use anyhow::anyhow;
use log::{debug, warn};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Adapter, BlendState, Buffer, BufferAddress, BufferUsages, Color, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, Device, DeviceDescriptor, Features, FragmentState, FrontFace,
    IndexFormat, Instance, Limits, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor,
    PolygonMode, PrimitiveState, PrimitiveTopology, Queue, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, RequestAdapterOptionsBase, ShaderModuleDescriptor, ShaderSource,
    Surface, SurfaceConfiguration, TextureFormat, TextureViewDescriptor, VertexAttribute,
    VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

use crate::model::{Mesh, Vertex};

const CLEAR_COLOR: wgpu::Color = Color {
    r: 0.85,
    g: 0.8,
    b: 1.0,
    a: 1.0,
};

pub struct Renderer {
    instance: Instance,
    surface: Surface,
    surface_config: SurfaceConfiguration,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl Renderer {
    pub async fn new(target: &winit::window::Window) -> anyhow::Result<Self> {
        let instance = Instance::default();
        let surface = unsafe { instance.create_surface(target) }?;
        let adapter = instance
            .request_adapter(&RequestAdapterOptionsBase {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .ok_or_else(|| anyhow!("Failed to request adapter."))?;
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("device"),
                    features: Features::empty(),
                    limits: Limits::default(),
                },
                None,
            )
            .await?;

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_capabilities.formats[0]);
        let surface_config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: target.inner_size().width,
            height: target.inner_size().height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        // let diffuse_bytes = include_bytes!("../res/cube.png");
        let render_pipeline =
            Self::create_render_pipeline(&device, include_str!("./shaders/shader.wgsl"));
        let circle_mesh = Mesh::create_circle(100).unwrap();
        let vertex_buffer = Self::create_vertex_buffer(&device, &circle_mesh.vertices);
        let index_buffer = Self::create_index_buffer(&device, &circle_mesh.indices);

        Ok(Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            surface_config,
            render_pipeline,
            vertex_buffer,
            index_buffer,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) -> anyhow::Result<()> {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
            Ok(())
        } else {
            return Err(anyhow!("size has to be greater than zero"));
        }
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        let output = self.surface.get_current_texture()?;
        let texture_view = output
            .texture
            .create_view(&TextureViewDescriptor::default());
        let mut command_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("command_encoder"),
            });

        {
            let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(CLEAR_COLOR),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);
            render_pass.draw_indexed(
                0..self.index_buffer.size() as u32 / std::mem::size_of::<u32>() as u32,
                0,
                0..1,
            )
        }

        self.queue.submit(std::iter::once(command_encoder.finish()));
        output.present();

        Ok(())
    }

    // assumes the entry points of the shader are vs_main and fs_main respectively
    fn create_render_pipeline(device: &Device, shader_source: &str) -> RenderPipeline {
        let shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("shader_module"),
            source: ShaderSource::Wgsl(std::borrow::Cow::from(shader_source)),
        });
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: None,
            vertex: VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[Vertex::vertex_buffer_layout()],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview: None,
        })
    }

    fn create_vertex_buffer(device: &Device, vertices: &[Vertex]) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        })
    }

    fn create_index_buffer(device: &Device, indices: &[u32]) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        })
    }
}
