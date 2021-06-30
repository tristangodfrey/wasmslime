#[derive(Clone, Copy)]
pub struct SensorConfig {
    /// The size of the area that a sensor observes (in pixels)
    pub width: usize,
    /// The angle between individual sensors
    pub angle: f64,
    /// The distance between the agent and the sensor location (in pixels)
    pub offset_distance: usize
}

impl Default for SensorConfig {
    fn default() -> Self {
        Self {
            width: 1,
            angle: 45f64,
            offset_distance: 9
        }
    }
}

pub struct SimulationConfig {
    /// Sensor properties
    pub sensor_config: SensorConfig,
    /// Distance an agent can move per step (in pixels)
    pub step_size: usize,
    /// Chemo-attractant deposition per step
    pub deposition: u8,
    /// Probability of a random change in direction (value between 0-1)
    pub cd_prob: f64,
    /// Sensitivity threshold
    pub s_min: usize,
    /// Width of the environment
    pub width: usize,
    /// Height of the environment
    pub height: usize,
    /// How far a cell will rotate when it detects something
    pub rotation_angle: f64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            sensor_config: SensorConfig::default(),
            step_size: 1,
            deposition: 255,
            cd_prob: 0f64,
            s_min: 50,
            width: 100,
            height: 100,
            rotation_angle: 45f64
        }
    }
}