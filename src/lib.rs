use std::f64;
use wasm_bindgen::prelude::*;
use js_sys::Error;

fn return_string_error(msg: &str) -> Result<String, JsValue> {
    Err(Error::new(msg).into())
}

#[wasm_bindgen(js_name=getCanvas)]
pub fn start(canvas_id: &str) -> Result<(), JsValue> {
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id(canvas_id);
  match canvas {
    Some(element) => {
      let canvas: web_sys::HtmlCanvasElement = element.dyn_into::<web_sys::HtmlCanvasElement>()?;
      let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
      context.set_fill_style(&JsValue::from_str("black"));
      context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
      context.set_fill_style(&JsValue::from_str("white"));
      context.fill_text("Hello, World!", 10.0, 50.0);
    }
    _ => {
      web_sys::console::log_1(&JsValue::from_str("No canvas found"));
      return_string_error("No canvas found")?;
    }
  }
  Ok(())
}

#[wasm_bindgen(js_name=getContext)]
pub fn get_context(w: u32, h: u32) -> Result<web_sys::OffscreenCanvasRenderingContext2d, JsValue> {
  // let document = web_sys::window().unwrap().document().unwrap();
  // let canvas = document.create_element("canvas")?;

  let canvas: web_sys::OffscreenCanvas = web_sys::OffscreenCanvas::new(w, h)?;

  // let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
  // let context = canvas
  //   .get_context("2d")?
  //   .unwrap()
  //   .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
  let context: web_sys::OffscreenCanvasRenderingContext2d = canvas.get_context("2d")?
    .unwrap()
    .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()?;
  // let data = canvas.to_data_url()?;
  // web_sys::console::log_1(&JsValue::from_str(&data));

  Ok(context)
}

#[wasm_bindgen(js_name=drawImageData)]
pub fn draw_img_data(ctx: &web_sys::OffscreenCanvasRenderingContext2d, bitmap: &web_sys::ImageBitmap) -> Result<(), JsValue> {
  ctx.canvas().set_width(bitmap.width() as u32);
  ctx.canvas().set_height(bitmap.height() as u32);
  ctx.draw_image_with_image_bitmap(bitmap, 0.0, 0.0)
}