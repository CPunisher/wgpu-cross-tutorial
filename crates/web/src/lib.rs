use app::App;
use wasm_bindgen::{JsCast, prelude::wasm_bindgen};

mod app;

#[wasm_bindgen(start)]
fn run() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct WgpuWrapper {
    app: App,
}

#[wasm_bindgen]
impl WgpuWrapper {
    #[wasm_bindgen(constructor)]
    pub async fn new(canvas_id: &str) -> Self {
        let window = web_sys::window().expect("Cannot get window");
        let document = window.document().expect("No document on window");
        let canvas: web_sys::HtmlCanvasElement = document
            .get_element_by_id(canvas_id)
            .and_then(|element| element.dyn_into().ok())
            .expect("Cannot get canvas by id");

        let width = canvas.width();
        let height = canvas.height();
        let app = App::init(canvas, width, height).await;
        Self { app }
    }

    pub fn render(&self) {
        self.app.render();
    }
}
