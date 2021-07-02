use rand::Rng;
use rand::random;
use rand::thread_rng;
use web_sys::ImageData;
use wasm_bindgen::Clamped;

use super::plane::*;
use super::config::*;
use super::point::*;
use super::trail_map::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Copy)]
pub struct Cell {
    /// The pixel position of the cell
    pub position: Point<f64>,
    /// Direction in which the cell is "pointed", this determines what is in its field of view
    pub direction: f64
}

impl Cell {
    pub fn position_discrete(&self) -> Point<usize> {
        self.position.into()
    }

    /// Updates the position of the cell according to the current direction + passed distance
    pub fn step(&mut self, distance: f64)
    {
        self.position = self.position + (Point::from_degrees(self.direction) * distance);
    }
}

#[derive(Clone)]
pub struct CellMap {
    pub cells: HashMap<Point<usize>, Cell>,
    width: usize,
    height: usize,
    sensor_config: SensorConfig
}

impl CellMap {
    pub fn new(width: usize, height: usize, sensor_config: SensorConfig) -> Self {

        let cells = HashMap::with_capacity(width * height);

        Self {
            cells,
            width,
            height,
            sensor_config
        }
    }

    pub fn new_random(width: usize, height: usize, sensor_config: SensorConfig, probability: f64) -> Self {
        let cell_n = width * height;
        let mut cells: HashMap<Point<usize>, Cell> = HashMap::with_capacity(cell_n);

        for y in 0..height {
            for x in 0..width {
                if random::<f64>() < probability {
                    let position = Point { x: x as f64, y: y as f64 };
                    let direction = random::<f64>() * 360f64;
                    
                    cells.insert(position.into(), Cell { position, direction });
                }
            }
        }

        Self {
            cells,
            width,
            height,
            sensor_config
        }
    }

    pub fn add_cell(&mut self, position: Point<f64>, direction: f64) {
        let discrete_pos: Point<usize> = position.into();

        self.cells.insert(discrete_pos, Cell { position, direction });
    }

    pub fn get_cell(&self, position: Point<usize>) -> Option<&Cell> {
        self.cells.get(&position)
    }

    pub fn live_cells(&self) -> usize {
        self.cells.len()
    }

    pub fn render(&self) -> ImageData {

        let mut data = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = self.cells.get(&Point { x, y }) {
                    data.append(&mut vec![255, 255, 255, 255]); //white
                } else {
                    data.append(&mut vec![0, 0, 0, 255]); //black
                }
            }
        }
        
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), self.width as u32, self.height as u32).unwrap()
    }
}

impl Iterator for CellMap {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
pub mod model_tests {

    use super::*;

    #[test]
    pub fn add_cell()
    {
        let mut cell_map = CellMap::new(20, 20, SensorConfig::default());
        cell_map.add_cell(Point::new(2f64, 4f64), 0f64);

        let res = cell_map.get_cell(Point::new(2, 4));

        assert!(res.is_some());
    }

}