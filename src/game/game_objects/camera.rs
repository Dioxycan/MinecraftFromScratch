use nalgebra_glm as glm;
use num::clamp;
#[derive(Debug)]
pub struct Camera {
    pub projection: glm::Mat4,
    pub view: glm::Mat4,
    pub rotation: glm::Vec3,
}
impl Camera {
    pub fn new() -> Self {
        Camera {
            projection: glm::Mat4::identity(),
            view: glm::Mat4::identity(),
            rotation: glm::Vec3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn set_orthographic_projection(
        &mut self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) {
        self.projection = glm::ortho_rh_zo(left, right, bottom, top, near, far);
    }
    pub fn set_perspective_projection(&mut self, fov: f32, aspect_ratio: f32, near: f32, far: f32) {
        self.projection = glm::perspective_lh_zo(aspect_ratio, fov, near, far);
    }
    pub fn set_view_direction(
        &mut self,
        position: &glm::Vec3,
        direction: &glm::Vec3,
        up: &glm::Vec3,
    ) {
        self.view = glm::look_at_lh(position, direction, up);
    }
    pub fn set_view_target(&mut self, position: &glm::Vec3, target: &glm::Vec3, up: &glm::Vec3) {
        self.view = glm::look_at_lh(position, target, up);
    }
    pub fn camera_yaw(&mut self, yaw: f32) {
        self.rotation.y = yaw;
    }
    pub fn camera_pitch(&mut self, pitch: f32) {
        let pitch = clamp(pitch, -89.0, 89.0);
        self.rotation.x = pitch;
    }
}
