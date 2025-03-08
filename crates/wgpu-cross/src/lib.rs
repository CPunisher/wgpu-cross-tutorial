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
        // 1. 选择图形驱动后端，创建 Instance。这里启用所有的后端根据实际需要加载
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    // 2. 利用 Instance 创建 wgpu Surface 的封装
    let surface = unsafe { instance.create_surface_unsafe(options.target).unwrap() };

    // 3. 利用 Instance 创建 Adapter
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    // 4. 利用 Adapter 创建 Device 和 Queue
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

    // 5. 创建 SurfaceConfig，用于配置 Surface 的像素颜色格式、长宽、Alpha Mode 等
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
