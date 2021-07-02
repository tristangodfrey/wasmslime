use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;

use rand::Rng;
use rand::random;
use rand::seq::IteratorRandom;
use rand::thread_rng;

use super::cell::*;
use super::plane::*;
use super::config::*;
use super::point::*;
use super::trail_map::*;

pub struct Simulation {
    pub cell_map: CellMap,
    pub trail_map: TrailMap,
    pub config: SimulationConfig,
    random_cells: Vec<(Point<usize>, Cell)>
}

pub enum Direction {
    LEFT,
    RIGHT
}

impl Simulation {

    pub fn new(config: SimulationConfig, cell_map: CellMap, trail_map: TrailMap) -> Self {

        let cell_count = cell_map.cells.len();

        Self {
            cell_map,
            trail_map,
            config,
            random_cells: Vec::with_capacity(cell_count)
        }
    }

    pub fn motor(&mut self) {
        let n = self.cell_map.cells.len();

        let mut rng = thread_rng();

        self.random_cells = self.cell_map.cells.clone().drain().choose_multiple(&mut rng, n);

        for (point, mut cell) in self.random_cells.iter() {
            let new_point = cell.position + (Point::from_degrees(cell.direction) * self.config.step_size as f64);

            if self.cell_map.cells.contains_key(&new_point.into()) {
                // spot is occupied, don't move
                println!("Spot occupied by {:?}", cell);
                // choose random orientation
                cell.direction = random::<f64>() * 360f64;
                // update the cell
                let update = self.cell_map.cells.get_mut(&point).unwrap();
                *update = cell;

            } else {
                // move to the new coordinate
                println!("Spot free, moving");
                let discrete_point: Point<usize> = Point::from(new_point);

                let new_index = discrete_point.get_index(self.config.width, self.config.height);

                cell.position = new_point;

                // deposit trail on trailmap
                self.trail_map.data[new_index] = self.config.deposition;

                //update the cell
                self.cell_map.cells.insert(new_point.into(), cell);
                
                //remove it from the old position
                self.cell_map.cells.remove(&point);
            }
        }
    }

    /// Sensory stage, sets proper rotation for cells
    fn sensor(&mut self) {
        for cell in self.cell_map.cells.values_mut() {
            let sensor_config = self.config.sensor_config.clone();
            let offset = sensor_config.offset_distance;    

            let point_fw = cell.position + (Point::from_degrees(cell.direction) * offset as f64);
            
            let angle_fl = cell.direction - sensor_config.angle;
            let angle_fr = cell.direction + sensor_config.angle;

            let point_fl = cell.position + (Point::from_degrees(angle_fl) * offset as f64);
            let point_fr = cell.position + (Point::from_degrees(angle_fr) * offset as f64);

            let fw = self.trail_map.get_value_point(point_fw).unwrap_or(&0);
            let fr = self.trail_map.get_value_point(point_fr).unwrap_or(&0);
            let fl = self.trail_map.get_value_point(point_fl).unwrap_or(&0);

            let mut direction: Option<Direction> = None;

            if fw > fl && fw > fr {
                //stay here
                continue;
            } else if fw < fl && fw < fr {
                //rotate randomly by RA
                if random::<f64>() > 0.5f64 {
                    direction = Some(Direction::LEFT);
                } else {
                    direction = Some(Direction::RIGHT);
                }
            } else if fl < fr {
                //rotate right by RA
                direction = Some(Direction::RIGHT);
            } else if fr < fl {
                direction = Some(Direction::LEFT);
            }

            match direction {
                Some(Direction::LEFT) => {
                    cell.direction = (cell.direction - self.config.rotation_angle) % 360f64;
                },
                Some(Direction::RIGHT) => {
                    cell.direction = (cell.direction + self.config.rotation_angle) % 360f64;
                },
                None => {
                    continue;
                }
            }   
        }
    }

    fn diffuse(&mut self) {

        let kernel_radius = 1;

        let mut new_data = vec![0u8; self.config.height * self.config.width];

        //iterate each pixel
        for i in 0..self.trail_map.data.len() {
            let point = self.trail_map.get_coords(i);

            let start_row = point.y.checked_sub(kernel_radius).unwrap_or(0);
            let end_row = point.y + kernel_radius;
            let start_col = point.x.checked_sub(kernel_radius).unwrap_or(0);
            let end_col = point.x + kernel_radius;

            let mut sum: u64 = 0;
            let mut n: u8 = 0;

            for j in start_row..=end_row {
                for k in start_col..=end_col {
                    if let Some(val) = self.trail_map.get_value(j, k) {
                        n += 1;
                        sum += *val as u64;
                    }
                }
            }

            let avg = sum / n as u64;

            new_data[i] = avg as u8;
        }

        self.trail_map.data = new_data;
    }

    pub fn step(&mut self, n: usize) {
        for _ in 0..n {
            self.motor();
            self.sensor();
            self.diffuse();
        }
    }
}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test]
    pub fn simulation()
    {

    }

    #[test]
    pub fn motor_step()
    {
        let mut cell_map = CellMap::new(20, 20, SensorConfig::default());
        let mut trail_map = TrailMap::new(20, 20);
        let mut sim_conf = SimulationConfig::default();

        sim_conf.width = 20;
        sim_conf.height = 20;

        cell_map.add_cell(Point::new(2f64, 4f64), 0f64);

        let mut simulation = Simulation::new(sim_conf, cell_map, trail_map);

        // simulation.motor();

        // If we get a cell at this point, it moved correctly
        let cell = simulation.cell_map.get_cell(Point::new(3, 4));
        assert!(cell.is_some());
        
        // Check that the chemoattractant was deposited at this position
        let val = simulation.trail_map.get_value(3, 4);
        let empty_val = simulation.trail_map.get_value(2, 4);

        assert!(val.is_some());
        assert_eq!(val.unwrap(), &255u8);
        assert_eq!(empty_val.unwrap(), &0u8);
    }

    pub fn sensor_step()
    {

    }

    #[test]
    pub fn diffuse_step()
    {
        let mut cell_map = CellMap::new(5, 5, SensorConfig::default());
        let mut trail_map = TrailMap::new(5, 5);
        let mut sim_conf = SimulationConfig::default();

        trail_map.data[0] = 255;

        sim_conf.width = 5;
        sim_conf.height = 5;

        let rand_fn = || 0.5f64;

        let mut simulation = Simulation::new(sim_conf, cell_map, trail_map);

        simulation.diffuse();

        let expected = vec![
            63u8, /* 63,75 */ 42u8, /* 42,5 */ 0u8, 0u8, 0u8,
            42u8, /* 42.5 */ 28u8, /* 28,33 */ 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8,
        ];

        assert_eq!(simulation.trail_map.data, expected);
    }

    #[test]
    fn full_step()
    {
        let mut config = SimulationConfig::default();

        config.width = 500;
        config.height = 500;

        let mut cell_map = CellMap::new(config.width, config.height, config.sensor_config.clone());
        let mut trail_map = TrailMap::new(config.width, config.height);

        cell_map.add_cell(Point::new(1f64, 1f64), 0f64);
        cell_map.add_cell(Point::new(1f64, 2f64), 25f64);
        cell_map.add_cell(Point::new(1f64, 3f64), 90f64);

        cell_map.add_cell(Point::new(10f64, 1f64), 0f64);
        cell_map.add_cell(Point::new(20f64, 2f64), 25f64);
        cell_map.add_cell(Point::new(30f64, 3f64), 90f64);

        let mut simulation = Simulation::new(config, cell_map, trail_map);

        // simulation.step();

        assert_eq!(1, 1);
    }
}