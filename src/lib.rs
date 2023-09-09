//a Imports
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

mod inner;
use inner::Inner;

//a CanvasArt - the external interface
//tp CanvasArt
/// A paint module that is attached to a Canvas element in an HTML
/// document, which uses mouse events etc to provide a simple paint
/// program
#[wasm_bindgen]
pub struct CanvasArt {
    inner: Rc<Inner>,
}

//ip CanvasArt
#[wasm_bindgen]
impl CanvasArt {
    //fp new
    /// Create a new CanvasArt attached to a Canvas HTML element,
    /// adding events to the canvas that provide the paint program
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> CanvasArt {
        let inner = Inner::new(canvas).unwrap();
        inner.add_closures().unwrap();
        Self { inner }
    }

    //mp shutdown
    /// Shut down the CanvasArt, removing any event callbacks for the canvas
    pub fn shutdown(&self) -> Result<(), JsValue> {
        self.inner.shutdown()
    }

    //mp fill
    /// Fill the canvas with transparent black
    pub fn fill(&self) {
        self.inner.fill()
    }
    //zz All done
}
