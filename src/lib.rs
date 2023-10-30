use std::f64;
use image::Rgba;
use wasm_bindgen::prelude::*;
use js_sys::{Error, Promise, Object};
use image::{ImageBuffer, Rgb, Pixel, imageops::flip_horizontal};
use imageproc::edges::canny;
use wasm_bindgen_futures::JsFuture;
use web_sys::ImageBitmap;
use serde::Deserialize;

use crate::effects::{inc_brightness, desc_brightness};
use crate::base::RawImage;

pub mod base;
pub mod effects;
pub mod helpers;

fn return_string_error(msg: &str) -> Result<String, JsValue> {
    Err(Error::new(msg).into())
}
#[wasm_bindgen]
#[derive(Deserialize)]
pub struct RenderOptions {
  brightness: u8,
}

#[wasm_bindgen]
impl RenderOptions {
  fn new(brightness: u8) -> RenderOptions {
    RenderOptions {
      brightness: brightness
    }
  }
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
      context.fill_text("Hello, World!", 10.0, 50.0).unwrap();
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

#[wasm_bindgen(js_name=drawImageEdge)]
pub fn draw_img_edge(bitmap: &web_sys::ImageBitmap, low_threshold: JsValue, high_threshold: JsValue) -> Result<Promise, JsValue> {
  let w = bitmap.width();
  let h = bitmap.height();
  let canvas = web_sys::OffscreenCanvas::new(w, h)?;
  let ctx = canvas.get_context("2d")?
    .unwrap()
    .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()?;

  ctx.draw_image_with_image_bitmap(bitmap, 0.0, 0.0).unwrap();

  web_sys::console::log_3(&JsValue::from_str("draw_img_bitmap"), &w.into(), &h.into());

  let mut img = ImageBuffer::new(w, h);
  let image_data = ctx.get_image_data(0.0, 0.0, w as f64, h as f64)?;
  let data = image_data.data();

  for (x, y, pixel) in img.enumerate_pixels_mut() {
    let r = data[(y * w + x) as usize * 4];
    let g = data[(y * w + x) as usize * 4 + 1];
    let b = data[(y * w + x) as usize * 4 + 2];
    let pix = Rgb([r, g, b]);

    *pixel = pix.to_luma();
  }

  web_sys::console::log_1(&JsValue::from_str("to grayscale"));

  let f32_val = low_threshold.as_f64().unwrap() as f32;
  let f32_val_h = high_threshold.as_f64().unwrap() as f32;

  let edges_image: ImageBuffer<image::Luma<u8>, Vec<u8>> = canny(&img, f32_val, f32_val_h);

  web_sys::console::log_1(&JsValue::from_str("done canny"));

  let (width, height) = edges_image.dimensions();
  
  let mut output_data = vec![0u8; width as usize * height as usize * 4];
    
  let mut i: usize = 0;
  let total = width as usize * height as usize;
  // Iterate through total pixels
  while i < total {
    // Get the pixel value
    let pixel = edges_image.get_pixel(i as u32 % width, i as u32 / width)[0];
    // Set the pixel value
    output_data[i * 4] = pixel;
    output_data[i * 4 + 1] = pixel;
    output_data[i * 4 + 2] = pixel;
    output_data[i * 4 + 3] = 255;
    i += 1;
  }

  web_sys::console::log_2(&data.len().into(), &output_data.len().into());

  // draw edges_rgb to ctx
  let edges_image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
    wasm_bindgen::Clamped(&mut output_data),
    width,
    height,
  )?;
  web_sys::console::log_1(&JsValue::from_str("convert edge data done"));
  let scope = js_sys::global().dyn_into::<web_sys::WorkerGlobalScope>().unwrap();
  // edge img to bitma: web_sys::ImageDatap
  let p:Promise  = scope.create_image_bitmap_with_image_data(&edges_image_data)?;
  Ok(p)
}

#[wasm_bindgen(js_name=flipImgBitmap)]
pub async fn flip(bitmap: &web_sys::ImageBitmap, render_options: JsValue) -> Result<ImageBitmap, JsValue> {

  let mut _brightness: u8 = 0;
  let mut _is_inc = true;

  let options: RenderOptions = serde_wasm_bindgen::from_value(render_options).unwrap();

  match options.brightness >= 50 {
    true => {
      _brightness = helpers::map_value((options.brightness - 50).into())
    },
    false => {
      _brightness = helpers::map_value((50 - options.brightness).into());
      _is_inc = false;
    },
  }

  let w = bitmap.width();
  let h = bitmap.height();
  let canvas = web_sys::OffscreenCanvas::new(w, h)?;
  let ctx = canvas.get_context("2d")?
    .unwrap()
    .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()?;

  ctx.draw_image_with_image_bitmap(bitmap, 0.0, 0.0).unwrap();

  let mut img = ImageBuffer::new(w, h);
  let image_data = ctx.get_image_data(0.0, 0.0, w as f64, h as f64)?;
  let data = image_data.data();

  for (x, y, pixel) in img.enumerate_pixels_mut() {
    let r = data[(y * w + x) as usize * 4];
    let g = data[(y * w + x) as usize * 4 + 1];
    let b = data[(y * w + x) as usize * 4 + 2];
    let a = data[(y * w + x) as usize * 4 + 3];
    let pix = Rgba([r, g, b, a]);

    *pixel = pix;
  }

  let pixels: Vec<u8> = img.as_raw().to_vec();
  let mut p_img = RawImage::new(pixels, w, h);
  
  // inc_brightness(&mut p_img, 100);
  if _is_inc {
    inc_brightness(&mut p_img, _brightness);
  } else {
    desc_brightness(&mut p_img, _brightness);
  }

  let brighten_img: ImageBuffer<Rgba<u8>, Vec<u8>> = p_img.to_image_buffer();

  let flipped = flip_horizontal(&brighten_img);
  let (width, height) = flipped.dimensions();

  let mut output_data = vec![0u8; width as usize * height as usize * 4];

  for (i, p) in flipped.pixels().into_iter().enumerate() {
    output_data[i * 4] = p.0[0];
    output_data[i * 4 + 1] = p.0[1];
    output_data[i * 4 + 2] = p.0[2];
    output_data[i * 4 + 3] = p.0[3];
  }

  let future = image_to_bitmap(output_data, width, height);
  let res = future.await?;
  Ok(res)
}


/// convert raw pixels vec data to ImageBitmap
pub async fn image_to_bitmap(mut img: Vec<u8>, width: u32, height: u32) -> Result<web_sys::ImageBitmap, JsValue> {

  let img_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
    wasm_bindgen::Clamped(&mut img),
    width,
    height,
  )?;
  
  let scope = js_sys::global().dyn_into::<web_sys::WorkerGlobalScope>().unwrap();

  let promise: Promise = scope.create_image_bitmap_with_image_data(&img_data)?;
  let result = JsFuture::from(promise).await?;
  Ok(result.into())
}