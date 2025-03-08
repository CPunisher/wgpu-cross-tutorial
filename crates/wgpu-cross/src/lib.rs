pub mod renderer;

pub struct InitWgpuOptions {
    pub target: wgpu::SurfaceTargetUnsafe,
    pub width: u32,
    pub height: u32,
}

pub struct WgpuContext {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

pub async fn init_wgpu(options: InitWgpuOptions) -> WgpuContext {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        // 1. Select graphics backends and create `Instance`. We enable all backends here.
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    // 2. Create wgpu `Surface` with instance
    let surface = unsafe { instance.create_surface_unsafe(options.target).unwrap() };

    // 3. Request `Adapter`` from instance
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    // 4. Request `Device` and `Queue` from adapter
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        )
        .await
        .unwrap();

    // 5. Create `SurfaceConfig` to config the pixel format, width and height, and alpha mode etc.
    let caps = surface.get_capabilities(&adapter);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: caps.formats[0],
        width: options.width,
        height: options.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    WgpuContext {
        surface,
        device,
        queue,
        config,
    }
}
