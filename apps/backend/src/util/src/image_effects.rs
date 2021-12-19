use image::DynamicImage::ImageRgba8;
use image::ImageBuffer;
use photon_rs::filters::*;
use photon_rs::native::open_image_from_bytes;
use std::fs;

use crate::Error;
#[derive(Debug)]
pub enum Attrs {
    Cali,
    Dramatic,
    Filter(&'static str),
    Firenze,
    Golden,
    Lix,
    Lofi,
    Neue,
    Obsidian,
    PastelPink,
    Ryo,
}

pub fn apply_effects(buf: Vec<u8>, attrs: Vec<Attrs>, save: String) -> Result<(), Error> {
    let mut image = open_image_from_bytes(&buf).unwrap();
    for attribute in attrs {
        match attribute {
            Attrs::Cali => cali(&mut image),
            Attrs::Dramatic => dramatic(&mut image),
            Attrs::Filter(overlay) => filter(&mut image, overlay),
            Attrs::Firenze => firenze(&mut image),
            Attrs::Golden => golden(&mut image),
            Attrs::Lix => lix(&mut image),
            Attrs::Lofi => lofi(&mut image),
            Attrs::Neue => neue(&mut image),
            Attrs::Obsidian => obsidian(&mut image),
            Attrs::PastelPink => pastel_pink(&mut image),
            Attrs::Ryo => ryo(&mut image),
        }
    }
    let raw_pixels = image.get_raw_pixels();
    let width = image.get_width();
    let height = image.get_height();

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynimage = ImageRgba8(img_buffer);
    dynimage
        .to_rgba8()
        .save(format!("{}.png", save))
        .map_err(|_| Error::DatabaseError)
        .ok();
    fs::copy(format!("{}.png", save), save)
        .map_err(|_| Error::DatabaseError)
        .ok();
    Ok(())
    // let width = image.get_width();
    // let height = image.get_height();

    // let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    // let dynimage = ImageRgba8(img_buffer);
    // dynimage.
    // dynimage.
    // img.get_raw_pixels()
    // save_image
    // img.into_bytes()
}
