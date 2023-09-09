//a Imports
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::f64;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

//a Inner (and ClosureSet)
//ti ClosureSet
/// A dictionary of event name to closure, of event listeners added to
/// (e.g.) a Canvas
///
/// The closure set entries can be dropped, once they have been
/// removed from the element they were attached to as listeners
type ClosureSet = HashMap<&'static str, Closure<dyn FnMut()>>;

//tp Inner
/// The actual CanvasArt paint structure, with canvas and rendering
/// context, state, and closures
pub struct Inner {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    pressed: Cell<bool>,
    closures: RefCell<ClosureSet>,
}

//ip Inner
impl Inner {
    //fp new
    /// Create a new Inner canvas paint structure given a Canvas element
    ///
    /// Does not add the event listeners (for no really good reason)
    pub fn new(canvas: HtmlCanvasElement) -> Result<Rc<Self>, JsValue> {
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        let pressed = Cell::new(false);
        let closures = HashMap::new().into();
        Ok(Rc::new(Self {
            canvas,
            context,
            pressed,
            closures,
        }))
    }

    //mp add_closures
    /// Add event listeners as required; they are also put into the
    /// ClosureSet so that they can be removed later, and the Inner
    /// (handled as a Rc<Inner>) will have its uses dropped as the
    /// Closures themselves are dropped; hence the Rc<Inner> should
    /// have no uses after this due to the event listeners that may
    /// have been added in the past.
    pub fn add_closures(self: &Rc<Self>) -> Result<(), JsValue> {
        {
            let inner = self.clone();
            self.add_closure("mousedown", move |event| inner.mouse_down(event))?;
        }
        {
            let inner = self.clone();
            self.add_closure("mouseup", move |event| inner.mouse_up(event))?;
        }
        {
            let inner = self.clone();
            self.add_closure("mousemove", move |event| inner.mouse_move(event))?;
        }
        Ok(())
    }

    //mp shutdown
    /// Remove all the event listeneres added (in the ClosureSet) and
    /// drop the closures
    ///
    /// This should be called prior to dropping the Inner so that it is not leaked.
    pub fn shutdown(&self) -> Result<(), JsValue> {
        let closures = self.closures.take();
        for (reason, closure) in closures.into_iter() {
            self.canvas
                .remove_event_listener_with_callback(reason, closure.as_ref().unchecked_ref())?
        }
        Ok(())
    }

    //mp fill
    /// Fill the canvas with transparent black
    pub fn fill(&self) {
        self.context.clear_rect(0., 0., 400., 400.);
    }

    //mi add_closure
    /// Add a single event listener to the canvas given a callback
    /// function (that should match that required in terms of
    /// arguments)
    fn add_closure<Args, F>(
        self: &Rc<Self>,
        reason: &'static str,
        callback: F,
    ) -> Result<(), JsValue>
    where
        F: FnMut(Args) + 'static,
        Args: wasm_bindgen::convert::FromWasmAbi + 'static,
    {
        let closure = Closure::<dyn FnMut(_)>::new(callback);
        self.canvas
            .add_event_listener_with_callback(reason, closure.as_ref().unchecked_ref())?;
        let closure = unsafe { std::mem::transmute::<_, Closure<dyn FnMut()>>(closure) };
        self.closures.borrow_mut().insert(reason, closure);
        Ok(())
    }

    //mi mouse_down
    /// The event handler for mouse being pressed
    fn mouse_down(&self, event: MouseEvent) {
        self.context.begin_path();
        self.context
            .move_to(event.offset_x() as f64, event.offset_y() as f64);
        self.pressed.set(true);
    }

    //mi mouse_move
    /// The event handler for mouse moving, whether the button is pressed or not
    fn mouse_move(&self, event: MouseEvent) {
        if self.pressed.get() {
            self.context
                .line_to(event.offset_x() as f64, event.offset_y() as f64);
            self.context.stroke();
            self.context.begin_path();
            self.context
                .move_to(event.offset_x() as f64, event.offset_y() as f64);
        }
    }

    //mi mouse_up
    /// The event handler for mouse being released
    fn mouse_up(&self, event: MouseEvent) {
        self.pressed.set(false);
        self.context.set_stroke_style(&("red".into()));
        self.context
            .line_to(event.offset_x() as f64, event.offset_y() as f64);
        self.context.stroke();
    }

    //zz All done
}
