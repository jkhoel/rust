use std::f64;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window()
        .unwrap()
        .document()
        .expect("Expected to be able to access the document field on the window object");

    let canvas = document
        .get_element_by_id("canvas")
        .expect("Unable to get element with id 'canvas'");

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .expect("Was unable to cast element to HtmlCanvasElement. Is HTML element a canvas?");

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // Start Drawing
    ctx.begin_path();

    // Draw outer circle
    ctx.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth
    // ctx.move_to(110.0, 75.0);
    // ctx.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();
    ctx.move_to(110.0, 75.0);
    ctx.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye
    ctx.move_to(65.0, 65.0);
    ctx.arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye
    ctx.move_to(95.0, 65.0);
    ctx.arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Render to context with the stroke method
    ctx.stroke();
}
