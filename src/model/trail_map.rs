use web_sys::ImageData;
use wasm_bindgen::Clamped;

use super::plane::*;

#[derive(Clone)]
pub struct TrailMap {
    pub data: Vec<u8>,
    width: usize,
    height: usize
}

impl Into<ImageData> for TrailMap {
    fn into(self) -> ImageData {

        let mut data = Vec::new();

        for (index, value) in self.data.iter().enumerate() {
            let mut pixel: Vec<u8> = vec![*value, *value, *value, 255];
            data.append(&mut pixel); //black
        }
        
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), self.width() as u32, self.height() as u32).unwrap()
    }
}

impl TrailMap {
    pub fn new(width: usize, height: usize) -> Self {

        let mut data: Vec<u8> = vec![0u8; width * height];

        Self {
            data,
            width,
            height
        }
    }

    pub fn new_random<F>(width: usize, height: usize, random_fn: &mut F) -> Self where F: FnMut() -> f64 {
        let mut data = vec![0u8; width * height];

        for (index, el) in data.iter_mut().enumerate() {
                let value = (random_fn)() * 255f64;

                *el = value.round() as u8;            
        }

        Self {
            data,
            width,
            height
        }
    }

    // Fills a circular area of a given size, with an optional gradient
    // pub fn fill(&mut self, point: Point, radius: f64) {

    // }
}

impl Plane<u8> for TrailMap {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn data(&self) -> &Vec<u8> {
        &self.data
    }
}