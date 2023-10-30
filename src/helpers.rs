use crate::{RawImage};
use image::DynamicImage::ImageRgba8;
use image::{DynamicImage, ImageBuffer};

pub fn dyn_image_from_raw(photon_image: &RawImage) -> DynamicImage {
  // convert a vec of raw pixels (as u8s) to a DynamicImage type
  let _len_vec = photon_image.raw_pixels.len() as u128;
  let raw_pixels = &photon_image.raw_pixels;
  let img_buffer = ImageBuffer::from_vec(
      photon_image.width,
      photon_image.height,
      raw_pixels.to_vec(),
  )
  .unwrap();
  ImageRgba8(img_buffer)
}

pub fn map_value(value: u32) -> u8 {
  ((value - 0_u32) * 255_u32 / 100_u32) as u8
}