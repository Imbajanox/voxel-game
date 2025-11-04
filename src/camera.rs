use glam::{Mat4, Vec3};

/// Camera for viewing the voxel world
pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,   // Rotation around Y axis (left/right)
    pub pitch: f32, // Rotation around X axis (up/down)
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    /// Creates a new camera with default settings
    pub fn new(aspect: f32) -> Self {
        Self {
            position: Vec3::new(0.0, 32.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            fov: 70.0_f32.to_radians(),
            aspect,
            near: 0.1,
            far: 1000.0,
        }
    }

    /// Gets the view matrix for the camera
    pub fn view_matrix(&self) -> Mat4 {
        let direction = self.forward();
        Mat4::look_at_rh(
            self.position,
            self.position + direction,
            Vec3::Y,
        )
    }

    /// Gets the projection matrix for the camera
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }

    /// Gets the forward direction vector
    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ).normalize()
    }

    /// Gets the right direction vector
    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::Y).normalize()
    }

    /// Moves the camera forward
    pub fn move_forward(&mut self, distance: f32) {
        let forward = Vec3::new(
            self.yaw.cos(),
            0.0,
            self.yaw.sin(),
        ).normalize();
        self.position += forward * distance;
    }

    /// Moves the camera backward
    pub fn move_backward(&mut self, distance: f32) {
        self.move_forward(-distance);
    }

    /// Moves the camera right
    pub fn move_right(&mut self, distance: f32) {
        self.position += self.right() * distance;
    }

    /// Moves the camera left
    pub fn move_left(&mut self, distance: f32) {
        self.move_right(-distance);
    }

    /// Moves the camera up
    pub fn move_up(&mut self, distance: f32) {
        self.position.y += distance;
    }

    /// Moves the camera down
    pub fn move_down(&mut self, distance: f32) {
        self.position.y -= distance;
    }

    /// Rotates the camera
    pub fn rotate(&mut self, yaw_delta: f32, pitch_delta: f32) {
        self.yaw += yaw_delta;
        self.pitch += pitch_delta;
        
        // Clamp pitch to prevent camera flipping
        self.pitch = self.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
    }

    /// Updates the aspect ratio (e.g., when window is resized)
    pub fn update_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let camera = Camera::new(16.0 / 9.0);
        assert_eq!(camera.position, Vec3::new(0.0, 32.0, 0.0));
        assert_eq!(camera.yaw, 0.0);
        assert_eq!(camera.pitch, 0.0);
    }

    #[test]
    fn test_camera_movement() {
        let mut camera = Camera::new(1.0);
        let initial_pos = camera.position;
        camera.move_forward(5.0);
        assert_ne!(camera.position, initial_pos);
    }

    #[test]
    fn test_pitch_clamping() {
        let mut camera = Camera::new(1.0);
        camera.rotate(0.0, 100.0); // Try to rotate beyond limit
        assert!(camera.pitch <= 89.0_f32.to_radians());
        assert!(camera.pitch >= -89.0_f32.to_radians());
    }
}
