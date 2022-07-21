use nalgebra_glm as glm;
pub struct Camera {
    pub projection: glm::Mat4,
    pub view: glm::Mat4,
}
impl Camera {
    pub fn new() -> Self {
        Camera {
            projection: glm::Mat4::identity(),
            view: glm::Mat4::identity(),
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
    pub fn set_perspective_projection(
        &mut self,
        fov: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) {
        self.projection = glm::perspective_rh_zo(fov, aspect_ratio, near, far);
    }
    pub fn set_view_direction(&mut self,position:&glm::Vec3, direction: &glm::Vec3, up: &glm::Vec3) {
        let sum = position + direction;
        self.view = glm::look_at_rh(position, &sum, up);
    }
    pub fn set_view_target(&mut self,position:&glm::Vec3, target: &glm::Vec3, up: &glm::Vec3) {
        self.view = glm::look_at_rh(position, target, up);
    }
    // pub fn setViewYXZ(&mut self,position:&glm::Vec3, rotation: &glm::Vec3) {
    //     let rotate = glm::rotate_y_vec4(&glm::rotate_x_vec4(&glm::rotate_z_vec4(&glm::Vec4::identity(), rotation.z), rotation.x), rotation.y);
    //     self.view = glm::translate(&rotate, -position);
    // }
}
