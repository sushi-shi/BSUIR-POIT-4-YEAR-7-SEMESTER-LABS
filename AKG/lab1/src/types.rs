use nalgebra::{Vector4, Vector3};


use crate::transformations;

#[derive(Debug)]
pub struct State {
    pub camera: CameraPosition,
    pub obj_file: ObjFile,
}

#[derive(Debug)]
pub struct CameraPosition {
    pub eye: Vector3<f64>,
    pub target: Vector3<f64>,
    pub up: Vector3<f64>,
}

impl Default for CameraPosition {
    fn default() -> Self {
        CameraPosition {
            eye: transformations::EYE,
            target: transformations::TARGET,
            up: transformations::UP,
        }
    }
}

#[derive(Debug)]
pub struct ObjFile {
    pub verticies: Vec<Vertex>,
    pub textures: Vec<VertexTexture>,
    pub normals: Vec<VertexNormal>,
    pub faces: Vec<Face>,
}

pub type Vertex = Vector4<f64>;
pub type VertexTexture = Vector3<f64>;
pub type VertexNormal = Vector3<f64>;

#[derive(Debug, Clone)]
pub struct Face {
    pub vec: Vec<(i64, Option<i64>, Option<i64>)>,
}

impl Face {
    pub fn new(vec: Vec<(i64, Option<i64>, Option<i64>)>) -> Self {
        Face {
            vec,
        }
    }

    pub fn concrete(self, 
        verticies: &Vec<Vertex>, 
        textures: &Vec<VertexTexture>, 
        normals: &Vec<VertexNormal>,
    ) -> Self {
        // self
        Face::new(
            self.vec.into_iter().map(|(vertex_idx, m_texture_idx, m_normal_idx)| {(
                if vertex_idx > 0 { vertex_idx } else { vertex_idx + verticies.len() as i64  + 1},
                m_texture_idx.map(|texture_idx| 
                    if texture_idx > 0 { texture_idx } else { texture_idx + textures.len() as i64 + 1 }),
                m_normal_idx.map(|normal_idx| 
                    if normal_idx > 0 { normal_idx } else { normal_idx + normals.len() as i64 + 1 }),
            )}).collect()
        )
    }
}
