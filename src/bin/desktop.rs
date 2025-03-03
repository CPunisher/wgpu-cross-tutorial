use std::sync::Arc;

use wgpu::Backends;
use wgpu_cross::{InitWgpuOptions, WgpuContext, init_wgpu, renderer::Renderer};
use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = WgpuDemo::default();
    event_loop.run_app(&mut app).unwrap();
}

#[derive(Default)]
struct WgpuDemo {
    window: Option<Arc<Window>>,
    context: Option<WgpuContext>,
    renderer: Option<Renderer>,
}

impl ApplicationHandler for WgpuDemo {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {}

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("WGPU Example");
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.window = Some(window.clone());

        let context = pollster::block_on(init_wgpu(InitWgpuOptions {
            surface: window.clone(),
            backends: Backends::all(),
            width: window.inner_size().width,
            height: window.inner_size().height,
        }));
        self.renderer = Some(Renderer::init(&context));
        self.context = Some(context)
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                window.pre_present_notify();

                let context = self.context.as_ref().unwrap();
                let output = context.surface.get_current_texture().unwrap();
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let renderer = self.renderer.as_ref().unwrap();
                renderer.render(view);
                output.present();
            }
            _ => (),
        }
    }
}
