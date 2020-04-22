/// Structure containing movement information
pub struct Movement {
    pub drag: f32,
    pub mass: f32,
    pub speed_x: f32,
    pub speed_y: f32,
    pub force_x: f32,
    pub force_y: f32,
    pub acceleration_x: f32,
    pub acceleration_y: f32
}

impl Movement {

    /// Create default movement struct with drag and mass
    pub fn new(drag: f32, mass: f32) -> Movement {
        Movement {
            drag: drag,
            mass: mass,
            speed_x: 0.0,
            speed_y: 0.0,
            force_x: 0.0,
            force_y: 0.0,
            acceleration_x: 0.0,
            acceleration_y: 0.0
        }
    }

    /// Update the physics of a movement struct 
    pub fn update(&mut self) {
        // Movement
        self.acceleration_x = (self.force_x - self.drag * self.speed_x) / self.mass;
        self.acceleration_y = (self.force_y - self.drag * self.speed_y) / self.mass;

        self.speed_x +=  self.acceleration_x;
        self.speed_y +=  self.acceleration_y;

    }
}
