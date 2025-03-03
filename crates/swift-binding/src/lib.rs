use std::ffi::c_void;

mod app;

#[repr(transparent)]
pub struct WgpuWrapper(*mut c_void);

#[unsafe(no_mangle)]
pub fn init_wgpu(metal_layer: *mut c_void, width: u32, height: u32) -> WgpuWrapper {
    let app = pollster::block_on(app::App::init(metal_layer, width, height));
    WgpuWrapper(Box::into_raw(Box::new(app)).cast())
}

#[unsafe(no_mangle)]
pub fn render(wrapper: WgpuWrapper) {
    let app = unsafe { &*(wrapper.0 as *mut app::App) };
    app.render();
}
