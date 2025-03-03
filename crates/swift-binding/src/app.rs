use std::ffi::c_void;

use wgpu_cross::{WgpuContext, renderer::Renderer};

pub(crate) struct App {
    context: WgpuContext,
    renderer: Renderer,
}

impl App {
    pub async fn init(metal_layer: *mut c_void, width: u32, height: u32) -> Self {
        let context = wgpu_cross::init_wgpu(wgpu_cross::InitWgpuOptions {
            surface: metal_layer,
            width,
            height,
        })
        .await;
        let renderer = Renderer::init(&context);

        Self { context, renderer }
    }

    pub fn render(&self) {
        let output = self.context.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.renderer.render(view);
        output.present();
    }
}
