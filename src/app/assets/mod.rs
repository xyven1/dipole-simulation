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
        meshes.insert("Axis".to_string(), Self::gen_axis());

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

    fn gen_axis() -> WebGLObject {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut normals = Vec::new();

        let x = Vector3::new(1.0, 0.0, 0.0);
        let y = Vector3::new(0.0, 1.0, 0.0);
        let z = Vector3::new(0.0, 0.0, 1.0);

        let x_end = x * 10.0;
        let y_end = y * 10.0;
        let z_end = z * 10.0;

        vertices.extend_from_slice(x_end.as_slice());
        vertices.extend_from_slice(y_end.as_slice());
        vertices.extend_from_slice(z_end.as_slice());

        indices.extend_from_slice(&[0, 1, 1, 2]);

        normals.extend_from_slice(x.as_slice());
        normals.extend_from_slice(y.as_slice());
        normals.extend_from_slice(z.as_slice());

        WebGLObject {
            vertices,
            indices,
            normals,
        }
    }
}
