use nalgebra::{Matrix4, Vector3};

use crate::types::Vertex;
use crate::types::CameraPosition;

// It is not a linear transformation
pub fn translation(tran_vec: Vector3<f64>) -> Matrix4<f64> {
    Matrix4::new(
        1.0, 0.0, 0.0, tran_vec[0],
        0.0, 1.0, 0.0, tran_vec[1],
        0.0, 0.0, 1.0, tran_vec[2],
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn scale(scale_vec: Vector3<f64>) -> Matrix4<f64> {
    Matrix4::new(
        scale_vec[0], 0.0, 0.0, 0.0,
        0.0, scale_vec[1], 0.0, 0.0,
        0.0, 0.0, scale_vec[2], 0.0,
        0.0, 0.0, 0.0, 1.0,  
    )
}


pub fn rotate_x(phi: f64) -> Matrix4<f64> {
    Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, phi.cos(), 0.0 - phi.sin(), 0.0,
        0.0, phi.sin(), phi.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0,  
    )
}

pub fn rotate_y(phi: f64) -> Matrix4<f64> {
    Matrix4::new(
        phi.cos(), 0.0, phi.sin(), 0.0,
        0.0, 1.0, 0.0, 0.0,
        -phi.sin(), 0.0, phi.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn rotate_z(phi: f64) -> Matrix4<f64> {
    Matrix4::new(
        phi.cos(), -phi.sin(), 0.0, 0.0,
        phi.sin(), phi.cos(), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}


/*
 * Позиция камеры (eye)
 * Позиция цели (target)
 * Вектор вверх от камеры (up)
 *
 */

pub const EYE: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
pub const TARGET: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
pub const UP: Vector3<f64> = Vector3::new(0.0, 0.0, 1.0);


pub fn view(eye: Vector3<f64>, target: Vector3<f64>, up: Vector3<f64>) -> Matrix4<f64> {
    let z_axis:  Vector3<f64> = (eye - target).normalize();
    let x_axis: Vector3<f64> = (z_axis.cross(&up)).normalize();
    let y_axis: Vector3<f64> = up;

    Matrix4::new(
        x_axis[0], x_axis[1], x_axis[2], -(x_axis.dot(&eye)),
        y_axis[0], y_axis[1], y_axis[2], -(y_axis.dot(&eye)),
        z_axis[0], z_axis[1], z_axis[2], -(z_axis.dot(&eye)),
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn projection_ortho(width: f64, height: f64, z_near: f64, z_far: f64) -> Matrix4<f64> {
    Matrix4::new(
        2.0 / width, 0.0, 0.0, 0.0,
        0.0, 2.0 / height, 0.0, 0.0,
        0.0, 0.0, 1.0 / (z_near - z_far), z_near / (z_near - z_far),
        0.0, 0.0, 0.0, 1.0,
    )
}


pub fn projection_persp(width: f64, height: f64, z_near: f64, z_far: f64) -> Matrix4<f64> {
    Matrix4::new(
        2.0 * z_near / width, 0.0, 0.0, 0.0,
        0.0, 2.0 * z_near / height, 0.0, 0.0,
        0.0, 0.0, z_far / (z_near - z_far), z_near * z_far / (z_near - z_far),
        0.0, 0.0, -1.0, 0.0,
    )
}

pub fn projection_persp_2(fov: f64, aspect: f64, z_near: f64, z_far: f64) -> Matrix4<f64> {
    Matrix4::new(
        1.0 / aspect * (fov / 2.0).tan(), 0.0, 0.0, 0.0,
        0.0, (fov / 2.0).tan(), 0.0, 0.0,
        0.0, 0.0, z_far / (z_near - z_far), z_near * z_far / (z_near - z_far),
        0.0, 0.0, -1.0, 0.0,
    )
}

pub fn viewport(width: f64, height: f64, x_min: f64, y_min: f64) -> Matrix4<f64> {
    Matrix4::new(
        width / 2.0, 0.0           , 0.0, x_min + width / 2.0 ,
        0.0        , - height / 2.0, 0.0, y_min + height / 2.0,
        0.0        , 0.0           , 1.0, 0.0                 ,
        0.0        , 0.0           , 0.0, 1.0                 ,
    )
}


// [VIEWPORT] x [PROJECTION] x [VIEW] x [MODEL]

pub fn transform(verticies: &Vec<Vertex>, camera: CameraPosition) -> Vec<(f64, f64)> {
    // verticies.map(|vertex| vertex * viewport

    todo!()
}
