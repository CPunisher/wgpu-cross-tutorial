use wgpu::{Device, Queue, TextureView};

use crate::WgpuContext;

pub struct Renderer {
    device: Device,
    queue: Queue,
}

impl Renderer {
    pub fn init(context: &WgpuContext) -> Self {
        Self {
            device: context.device.clone(),
            queue: context.queue.clone(),
        }
    }

    pub fn render(&self, view: TextureView) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        }
        self.queue.submit(Some(encoder.finish()));
    }
}
