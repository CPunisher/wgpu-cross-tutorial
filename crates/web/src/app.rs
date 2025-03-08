use std::ptr::NonNull;

use web_sys::HtmlCanvasElement;
use wgpu::rwh::{RawDisplayHandle, WebCanvasWindowHandle, WebDisplayHandle};
use wgpu_cross::{WgpuContext, renderer::Renderer};

pub(crate) struct App {
    _canvas: HtmlCanvasElement,
    context: WgpuContext,
    renderer: Renderer,
}

impl App {
    pub async fn init(canvas: HtmlCanvasElement, width: u32, height: u32) -> Self {
        let context = wgpu_cross::init_wgpu(wgpu_cross::InitWgpuOptions {
            // Create handles from canvas element
            target: wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_display_handle: {
                    let handle = WebDisplayHandle::new();
                    RawDisplayHandle::Web(handle)
                },
                raw_window_handle: {
                    let obj: NonNull<core::ffi::c_void> = NonNull::from(&canvas).cast();
                    let handle = WebCanvasWindowHandle::new(obj);
                    wgpu::rwh::RawWindowHandle::WebCanvas(handle)
                },
            },
            width,
            height,
        })
        .await;
        let renderer = Renderer::init(&context);

        Self {
            _canvas: canvas,
            context,
            renderer,
        }
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
