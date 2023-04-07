use anyhow::anyhow;
use wgpu::{
    Adapter, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance, Limits,
    LoadOp, Operations, Queue, RenderPassDescriptor, RequestAdapterOptionsBase, Surface,
    SurfaceConfiguration, TextureViewDescriptor,
};

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
                    features: Features::default(),
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

        Ok(Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            surface_config,
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
            let _render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
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
        }

        self.queue.submit(std::iter::once(command_encoder.finish()));
        output.present();

        Ok(())
    }
}
