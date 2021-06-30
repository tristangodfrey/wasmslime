use super::cell::*;
use super::plane::*;
use super::config::*;
use super::point::*;
use super::trail_map::*;
use super::super::rand;

pub struct Simulation {
    pub cell_map: CellMap,
    pub trail_map: TrailMap,
    cell_vec: Vec<Option<Cell>>,
    pub config: SimulationConfig,
    random_fn: fn() -> f64
}

pub enum Direction {
    LEFT,
    RIGHT
}

impl Simulation {

    pub fn new(config: SimulationConfig, cell_map: CellMap, trail_map: TrailMap, random_fn: fn() -> f64) -> Self {
        Self {
            cell_map,
            trail_map,
            cell_vec: vec![None; config.height * config.width],
            config,
            random_fn
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.config.width) + x
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&Option<Cell>> {
        self.cell_vec.get(self.get_index(x, y))
    }

    pub fn motor(&mut self) {
        // We create a vector holding every single index of the cell map
        let mut remaining: Vec<usize> = (0..self.cell_map.cells.len()).collect();

        while ! remaining.is_empty() {

            let ri = (self.random_fn)() * ((remaining.len() - 1) as f64);

            let ri = ri.round() as usize;

            let i = remaining[ri];

            remaining.remove(ri);

            if let Some(mut cell) = self.cell_map.cells.get(i).unwrap().clone() {
                let new_point = cell.position + (Point::from_degrees(cell.direction) * self.config.step_size as f64);

                println!("New point = {:?}", new_point);

                //attempt to move it
                let new_point_discrete: Point<usize> = new_point.into();

                if let Some(option) = self.cell_map.get_cell(new_point_discrete.x, new_point_discrete.y) { //checks against index out of bounds
                    if let Some(cell) = option {
                        // spot is occupied, don't move
                        println!("Spot occupied by {:?}", cell);
                        // choose random orientation
                        let new_cell = Cell { direction: ((self.random_fn)() * 360f64), position: cell.position };
                        self.cell_map.cells[i] = Some(new_cell);
                        
                    } else {
                        // move to the new coordinate
                        println!("Spot free, moving");
                        let new_index = self.cell_map.get_index(new_point_discrete.x, new_point_discrete.y);

                        cell.position = new_point;

                        self.cell_map.cells[new_index] = Some(cell);
                        // deposit trail on trailmap
                        self.trail_map.data[new_index] = self.config.deposition;
                        //remove the old cell
                        self.cell_map.cells[i] = None;
                    }
                }
            }
        }
    }

    /// Sensory stage, sets proper rotation for cells
    fn sensor(&mut self) {
        for i in 0..self.cell_map.cells.len() {
            if let Some(cell) = self.cell_map.cells.get_mut(i).unwrap() {
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
                    if (self.random_fn)() > 0.5f64 {
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

    pub fn step(&mut self) {
        self.motor();
        self.sensor();
        self.diffuse();
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

        let mut simulation = Simulation::new(sim_conf, cell_map, trail_map, rand);

        simulation.motor();

        // If we get a cell at this point, it moved correctly
        let cell = simulation.cell_map.get_value(3, 4);
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

        let mut simulation = Simulation::new(sim_conf, cell_map, trail_map, rand_fn);

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

        let mut simulation = Simulation::new(config, cell_map, trail_map, || 0.3f64);

        simulation.step();

        assert_eq!(1, 1);
    }
}