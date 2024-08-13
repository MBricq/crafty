use std::f32::consts::PI;
use std::time::Duration;
use crate::chunk::CHUNK_FLOOR;
use crate::gravity::GravityHandler;
use crate::vector::Vector3;
use crate::world::World;

/// Travel speed [m/s] or [cube/s]
const SPEED: f32 = 2.0; 

pub const PLAYER_HEIGHT: f32 = 2.;

pub enum MotionState {
    W,
    S,
    A,
    D,
    None,
}

/// First player camera
/// The state includes the position and the speed
pub struct Camera<'a> {
    /// Position of the camera
    position: Vector3,
    
    /// Orientation of the camera Yaw, Pitch
    rotation: [f32; 2],
    
    // state: MotionState
    w_pressed: bool,
    s_pressed: bool,
    a_pressed: bool,
    d_pressed: bool,
    
    /// Reference to the world is necessary for collision detection.
    world: &'a World,
    
    /// For handling free-fall
    gravity_handler: GravityHandler
}

impl<'a> Camera<'a> {
    /// based on right hand perspective look along the positive z-Axis
    // pub fn new(collision_callback: impl FnMut([f32;3]) -> bool + 'a) -> Self {
    pub fn new(world: &'a World) -> Self {
        Self {
            position: Vector3::new(4.0, CHUNK_FLOOR as f32 + PLAYER_HEIGHT, 3.0),
            rotation: [PI, 0.0],
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
            world,
            gravity_handler: GravityHandler::new()
        }
    }

    pub fn step(&mut self, elapsed: Duration) {
        // Compute the next position
        let f = self.ground_direction_forward();
        let l = self.ground_direction_right();
        let mut next_pos = self.position.clone();
        let mut next_pos_amplified = self.position.clone();
        let amplitude = SPEED * elapsed.as_secs_f32();
        let ratio = 10.;
        if self.w_pressed {
            next_pos += f * amplitude;
            next_pos_amplified += f * amplitude * ratio
        }
        if self.s_pressed {
            next_pos -= f * amplitude;
            next_pos_amplified -= f * amplitude * ratio
        }
        if self.d_pressed {
            next_pos += l * amplitude;
            next_pos_amplified += l * amplitude * ratio
        }
        if self.a_pressed {
            next_pos -= l * amplitude;
            next_pos_amplified -= l * amplitude * ratio
        }
        
        // Collision detection (xz-plane)
        let is_free = self.world.is_position_free(&next_pos_amplified);
        
        // Free-fall handling
        let is_falling = self.world.is_position_free_falling(&next_pos_amplified);
        let dz_fall = self.gravity_handler.step(is_falling, elapsed);
        next_pos[1] -= dz_fall;;

        // Position update
        if is_free {
            self.position = next_pos
        }
        // println!("free={is_free}, pos={next_pos:?}, tested={next_pos_amplified:?}");
    }

    pub fn toggle_state(&mut self, state: MotionState) {
        match state {
            MotionState::W => self.w_pressed = !self.w_pressed,
            MotionState::S => self.s_pressed = !self.s_pressed,
            MotionState::A => self.a_pressed = !self.a_pressed,
            MotionState::D => self.d_pressed = !self.d_pressed,
            MotionState::None => {}
        }
    }
    
    pub fn jump(&mut self) {
        self.gravity_handler.jump();
    }

    pub fn up(&mut self) {
        self.position[1] += 1.;
    }

    pub fn down(&mut self) {
        self.position[1] -= 1.;
    }

    /// Returns the normalized direction vector
    fn direction(&self) -> Vector3 {
        let yaw = self.rotation[0];
        let pitch = self.rotation[1];
        Vector3::new(yaw.cos() * pitch.cos(), pitch.sin(), yaw.sin() * pitch.cos())
    }

    fn ground_direction_forward(&self) -> Vector3 {
        Vector3::new(self.rotation[0].cos(), 0., self.rotation[0].sin())
    }

    fn ground_direction_right(&self) -> Vector3 {
        Vector3::new(self.rotation[0].sin(), 0., -self.rotation[0].cos())
    }

    /// Returns the view matrix, from the given camera parameters
    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        // Compute the normalised direction vector
        let forward = self.direction();
        let camera_up = Vector3::new(0., 1., 0.);
        let mut s = camera_up.cross(&forward);
        s.normalize();
        let u = forward.cross(&s);
        let p = [-self.position[0] * s[0] - self.position[1] * s[1] - self.position[2] * s[2],
            -self.position[0] * u[0] - self.position[1] * u[1] - self.position[2] * u[2],
            -self.position[0] * forward[0] - self.position[1] * forward[1] - self.position[2] * forward[2]];
        [
            [s[0], u[0], forward[0], 0.0],
            [s[1], u[1], forward[1], 0.0],
            [s[2], u[2], forward[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }

    pub fn mousemove(&mut self, horizontal: f32, vertical: f32, sensitivity: f32) {
        self.rotation[0] -= horizontal * sensitivity;

        // don't let the player turn upside down
        if vertical > 0.0 && self.rotation[1] < PI * 0.5 {
            self.rotation[1] += vertical * sensitivity;
        } else if vertical < 0.0 && self.rotation[1] > -PI * 0.5 {
            self.rotation[1] += vertical * sensitivity;
        }
    }
}