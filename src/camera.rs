use cgmath::{perspective, Deg, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3};

use crate::{
    input_manager::{InputManager, KeyCode},
    timing::Timing,
};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub eye: Point3<f32>,
    pub direction: Vector3<f32>,
    pub up: Vector3<f32>,
    pub fovy: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        // let view = Matrix4::look_to_rh(self.eye, self.target, self.up);
        let view = Matrix4::look_to_rh(self.eye, self.direction, self.up);
        let projection = perspective(Deg(self.fovy), self.aspect, self.near, self.far);
        projection * view * OPENGL_TO_WGPU_MATRIX
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_projection_matrix: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_projection_matrix: Matrix4::identity().into(),
        }
    }

    pub fn update(&mut self, camera: &Camera) {
        self.view_projection_matrix = camera.build_view_projection_matrix().into()
    }
}

pub struct CameraController {
    speed: f32,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }

    pub fn update_camera(&mut self, camera: &mut Camera, input: &InputManager, timing: &Timing) {
        if input.is_key_pressed(KeyCode::W) {
            camera.eye += camera.direction * self.speed * timing.time_delta().as_secs_f32();
        }
        if input.is_key_pressed(KeyCode::S) {
            camera.eye -= camera.direction * self.speed * timing.time_delta().as_secs_f32();
        }
        if input.is_key_pressed(KeyCode::A) {
            camera.eye.x += self.speed * timing.time_delta().as_secs_f32();
        }
        if input.is_key_pressed(KeyCode::D) {
            camera.eye.x -= self.speed * timing.time_delta().as_secs_f32();
        }
        if input.is_key_pressed(KeyCode::Space) {
            camera.eye.y += self.speed * timing.time_delta().as_secs_f32();
        }
        if input.is_key_pressed(KeyCode::LShift) {
            camera.eye.y -= self.speed * timing.time_delta().as_secs_f32();
        }
    }
}
