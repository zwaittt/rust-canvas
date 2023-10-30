use serde::{Deserialize, Serialize};
use image::{DynamicImage::ImageRgba8, ImageBuffer, Rgb, Rgba};
use base64::{Engine, engine::general_purpose};
use crate::helpers::dyn_image_from_raw;
use web_sys::{Blob, ImageData};
use std::io::Cursor;

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RawImage {
    pub raw_pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
impl RawImage {
    /// Create a new RawImage from a Vec of u8s, which represent raw pixels.
    pub fn new(raw_pixels: Vec<u8>, width: u32, height: u32) -> RawImage {
        RawImage {
            raw_pixels,
            width,
            height,
        }
    }

    /// Create a new RawImage from a base64 string.
    pub fn new_from_base64(base64: &str) -> RawImage {
        base64_to_image(base64)
    }

    /// Create a new RawImage from a byteslice.
    pub fn new_from_byteslice(vec: Vec<u8>) -> RawImage {
        let slice = vec.as_slice();

        let img = image::load_from_memory(slice).unwrap();

        let raw_pixels = img.to_rgba8().to_vec();

        RawImage {
            raw_pixels,
            width: img.width(),
            height: img.height(),
        }
    }

    /// Create a new RawImage from a Blob/File.
    pub fn new_from_blob(blob: Blob) -> RawImage {
        let bytes: js_sys::Uint8Array = js_sys::Uint8Array::new(&blob);

        let vec = bytes.to_vec();

        RawImage::new_from_byteslice(vec)
    }

    /// Create a new RawImage from a HTMLImageElement
    // #[cfg(feature = "web-sys")]
    // pub fn new_from_image(image: HtmlImageElement) -> RawImage {
    //     set_panic_hook();

    //     let document = web_sys::window().unwrap().document().unwrap();

    //     let canvas = document
    //         .create_element("canvas")
    //         .unwrap()
    //         .dyn_into::<web_sys::HtmlCanvasElement>()
    //         .unwrap();

    //     canvas.set_width(image.width());
    //     canvas.set_height(image.height());

    //     let context = canvas
    //         .get_context("2d")
    //         .unwrap()
    //         .unwrap()
    //         .dyn_into::<CanvasRenderingContext2d>()
    //         .unwrap();

    //     context
    //         .draw_image_with_html_image_element(&image, 0.0, 0.0)
    //         .unwrap();

    //     open_image(canvas, context)
    // }

    // pub fn new_from_buffer(buffer: &Buffer, width: u32, height: u32) -> RawImage {
    //     // Convert a Node.js Buffer into a Vec<u8>
    //     let raw_pixels: Vec<u8> = Uint8Array::new_with_byte_offset_and_length(
    //         &buffer.buffer(),
    //         buffer.byte_offset(),
    //         buffer.length(),
    //     ).to_vec();

    //     RawImage {
    //         raw_pixels,
    //         width,
    //         height,
    //     }
    // }

    /// Get the width of the RawImage.
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get the RawImage's pixels as a Vec of u8s.
    pub fn get_raw_pixels(&self) -> Vec<u8> {
        self.raw_pixels.clone()
    }

    /// Get the height of the RawImage.
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Convert the RawImage to base64.
    pub fn get_base64(&self) -> String {
        let mut img = dyn_image_from_raw(self);
        img = ImageRgba8(img.to_rgba8());

        // let mut buffer = BufWriter::new(vec![]);
        let mut buffer = Cursor::new(vec![]);
        img.write_to(&mut buffer, image::ImageOutputFormat::Png)
            .unwrap();
        let base64 = general_purpose::STANDARD.encode(&buffer.into_inner());

        let res_base64 = format!("data:image/png;base64,{}", base64.replace("\r\n", ""));

        res_base64
    }

    /// Convert the RawImage to raw bytes. Returns PNG.
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut img = dyn_image_from_raw(self);
        img = ImageRgba8(img.to_rgba8());
        let mut buffer = Cursor::new(vec![]);
        img.write_to(&mut buffer, image::ImageOutputFormat::Png)
            .unwrap();
        buffer.into_inner()
    }

    /// Convert the RawImage to raw bytes. Returns a JPEG.
    pub fn get_bytes_jpeg(&self, quality: u8) -> Vec<u8> {
        let mut img = dyn_image_from_raw(self);
        img = ImageRgba8(img.to_rgba8());
        let mut buffer = Cursor::new(vec![]);
        let out_format = image::ImageOutputFormat::Jpeg(quality);
        img.write_to(&mut buffer, out_format).unwrap();
        buffer.into_inner()
    }

    ///// Convert the RawImage's raw pixels to JS-compatible ImageData.
    #[cfg(all(feature = "web-sys", feature = "wasm-bindgen"))]
    #[allow(clippy::unnecessary_mut_passed)]
    pub fn get_image_data(&mut self) -> ImageData {
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut self.raw_pixels),
            self.width,
            self.height,
        )
        .unwrap()
    }

    ///// Convert ImageData to raw pixels, and update the RawImage's raw pixels to this.
    #[cfg(feature = "web-sys")]
    pub fn set_imgdata(&mut self, img_data: ImageData) {
        let width = img_data.width();
        let height = img_data.height();
        let raw_pixels = to_raw_pixels(img_data);
        self.width = width;
        self.height = height;
        self.raw_pixels = raw_pixels;
    }

    pub fn to_image_buffer(& mut self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let buf = self.get_raw_pixels().to_vec();
        let img_buffer:ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::from_raw(self.width, self.height, buf).unwrap();
        img_buffer
    }
}

pub fn base64_to_image(base64: &str) -> RawImage {
    let base64_to_vec: Vec<u8> = base64_to_vec(base64);

    let slice = base64_to_vec.as_slice();

    let mut img = image::load_from_memory(slice).unwrap();
    img = ImageRgba8(img.to_rgba8());
    let raw_pixels = img.to_bytes();

    RawImage {
        raw_pixels,
        width: img.width(),
        height: img.height(),
    }
}

pub fn base64_to_vec(base64: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(base64).unwrap()
}

#[cfg(feature = "web-sys")]
pub fn to_raw_pixels(imgdata: ImageData) -> Vec<u8> {
    imgdata.data().to_vec()
}