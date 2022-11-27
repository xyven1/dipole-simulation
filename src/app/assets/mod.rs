use bincode;
use blender_mesh::BlenderMesh;
use std::collections::HashMap;

#[derive(Default)]
pub struct Assets {
    meshes: HashMap<String, BlenderMesh>,
}

impl Assets {
    pub fn new() -> Assets {
        let meshes = Assets::download_meshes();

        Assets { meshes }
    }

    // In a real application you would download via XHR or fetch request, but here we just
    // included_bytes! for simplicity
    fn download_meshes() -> HashMap<String, BlenderMesh> {
        let meshes = include_bytes!("../../../meshes.bytes");
        let mut meshes: HashMap<String, BlenderMesh> = bincode::deserialize(meshes).unwrap();

        for (mesh_name, mesh) in meshes.iter_mut() {
            web_sys::console::log_1(&mesh_name.to_string().into());

            mesh.combine_vertex_indices();
            mesh.triangulate();

            if let Some(_armature_name) = mesh.armature_name.as_ref() {
                mesh.set_groups_per_vertex(4);
            } else {
                mesh.y_up();
            }
        }

        meshes
    }

    // In a real application you would download via XHR or fetch request, but here we just

    pub fn get_mesh(&self, mesh_name: &str) -> Option<&BlenderMesh> {
        self.meshes.get(mesh_name)
    }
}
