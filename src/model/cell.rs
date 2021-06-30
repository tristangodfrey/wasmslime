use web_sys::ImageData;
use wasm_bindgen::Clamped;

use super::plane::*;
use super::config::*;
use super::point::*;
use super::trail_map::*;

#[derive(Clone, Debug)]
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
}

#[derive(Clone)]
pub struct CellMap {
    pub cells: Vec<Option<Cell>>,
    width: usize,
    height: usize,
    sensor_config: SensorConfig
}

impl Plane<Option<Cell>> for CellMap {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn data(&self) -> &Vec<Option<Cell>> {
        &self.cells
    }
}

impl Into<ImageData> for CellMap {
    fn into(self) -> ImageData {

        let mut data = Vec::new();

        for (index, cell) in self.cells.iter().enumerate() {
            if let Some(cell) = cell {
                data.append(&mut vec![255, 255, 255, 255]); //white
            } else {
                
                data.append(&mut vec![0, 0, 0, 255]); //black
            }
        }
        
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), self.width() as u32, self.height() as u32).unwrap()
    }
}

impl CellMap {
    pub fn new(width: usize, height: usize, sensor_config: SensorConfig) -> Self {

        let cells = vec![Option::None; width * height];

        Self {
            cells,
            width,
            height,
            sensor_config
        }
    }

    pub fn new_random(width: usize, height: usize, sensor_config: SensorConfig, random_fn: fn() -> f64, probability: f64) -> Self {
        let mut cells = vec![Option::None; width * height];

        for (index, el) in cells.iter_mut().enumerate() {
            if (random_fn)() < probability {
                let y = index / width;
                let x = index % width;

                let position = Point { x: x as f64, y: y as f64 };
                let direction = (random_fn)() * 360f64;

                *el = Some(Cell { position, direction });
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

        let index = self.get_index(discrete_pos.x, discrete_pos.y);

        self.cells[index] = Some(Cell { position, direction });
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Option<Cell>> {
        let index = (y * self.width) + x;

        self.cells.get(index)
    }

    pub fn live_cells(&self) -> usize {
        self.cells.iter().filter(|val| val.is_some()).count()
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

        let res = cell_map.get_value(2, 4);

        assert!(res.is_some());
    }

}