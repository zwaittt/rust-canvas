use crate::base::RawImage;

pub fn inc_brightness(photon_image: &mut RawImage, brightness: u8) {
  let end = photon_image.get_raw_pixels().len() - 4;
  for i in (0..end).step_by(4) {
    let r_val = photon_image.raw_pixels[i];
    let g_val = photon_image.raw_pixels[i + 1];
    let b_val = photon_image.raw_pixels[i + 2];

    if r_val <= 255 - brightness {
      photon_image.raw_pixels[i] += brightness;
    } else {
      photon_image.raw_pixels[i] = 255;
    }
    if g_val <= 255 - brightness {
      photon_image.raw_pixels[i + 1] += brightness;
    } else {
      photon_image.raw_pixels[i + 1] = 255
    }

    if b_val <= 255 - brightness {
      photon_image.raw_pixels[i + 2] += brightness;
    } else {
      photon_image.raw_pixels[i + 2] = 255
    }
  }
}

pub fn desc_brightness(photon_image: &mut RawImage, brightness: u8) {
  let end = photon_image.get_raw_pixels().len() - 4;
  for i in (0..end).step_by(4) {
    let r_val = photon_image.raw_pixels[i];
    let g_val = photon_image.raw_pixels[i + 1];
    let b_val = photon_image.raw_pixels[i + 2];

    if r_val <= brightness {
      photon_image.raw_pixels[i] = 0;
    } else {
      photon_image.raw_pixels[i] -= brightness;
    }
    if g_val <= brightness {
      photon_image.raw_pixels[i + 1] = 0;
    } else {
      photon_image.raw_pixels[i + 1] -= brightness
    }

    if b_val <= brightness {
      photon_image.raw_pixels[i + 2] = 0;
    } else {
      photon_image.raw_pixels[i + 2] -= brightness;
    }
  }
}