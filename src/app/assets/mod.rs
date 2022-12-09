use std::collections::HashMap;

use nalgebra::Vector3;

use crate::{generate_sphere, webgl_object::WebGLObject};

#[derive(Default)]
pub struct Assets {
    meshes: HashMap<String, WebGLObject>,
}

impl Assets {
    pub fn new() -> Assets {
        let mut meshes: HashMap<String, WebGLObject> = HashMap::new();

        meshes.insert("Sphere".to_string(), Self::gen_sphere());

        Assets { meshes }
    }

    fn gen_sphere() -> WebGLObject {
        let sphere = generate_sphere::Polyhedron::new_isocahedron(0.5, 1);
        WebGLObject {
            vertices: sphere
                .positions
                .iter()
                .flat_map(|v| vec![v.0[0], v.0[1], v.0[2]])
                .collect(),
            indices: sphere
                .cells
                .iter()
                .flat_map(|v| vec![v.a as u16, v.b as u16, v.c as u16])
                .collect(),
            normals: sphere
                .normals
                .iter()
                .flat_map(|v| vec![v.0[0], v.0[1], v.0[2]])
                .collect(),
        }
    }

    pub fn gen_line(start: Vector3<f32>, end: Vector3<f32>) -> WebGLObject {
        WebGLObject {
            vertices: vec![start.x, start.y, start.z, end.x, end.y, end.z],
            indices: vec![],
            normals: vec![],
        }
    }

    // In a real application you would download via XHR or fetch request, but here we just

    pub fn get_mesh(&self, mesh_name: &str) -> Option<&WebGLObject> {
        self.meshes.get(mesh_name)
    }
}
