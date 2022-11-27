use crate::generate_sphere;
use crate::render::Flat;
use crate::render::FlatRenderOpts;
use crate::render::Mesh;
use crate::render::MeshRenderOpts;
use crate::render::Render;
use crate::render::WebRenderer;
use crate::shader::ShaderKind;
use crate::Assets;
use crate::State;
use nalgebra::Vector3;
use web_sys::WebGlRenderingContext as GL;

impl WebRenderer {
    pub(in crate::render) fn render_meshes(
        &self,
        gl: &GL,
        state: &State,
        assets: &Assets,
        clip_plane: [f32; 4],
        flip_camera_y: bool,
    ) {
        if !state.show_scenery() {
            return;
        }

        let time = state.clock();

        let x = (time * state.time_scale() / 1000.0).sin() * 2.0;
        let y = (time * state.time_scale() / 1000.0).cos() * 2.0;
        let z = 0.0;

        // Render Spheres
        let shader = self.shader_sys.get_shader(&ShaderKind::Flat).unwrap();
        self.shader_sys.use_program(gl, ShaderKind::Flat);

        let mesh_opts = FlatRenderOpts {
            pos: (x, y, z),
            flip_camera_y,
            color: Vector3::new(0., 0.0, 1.),
            as_lines: false,
        };
        let mesh_name = "Sphere";

        let sphere = Flat {
            object: assets.get_mesh(mesh_name).expect("Sphere"),
            shader,
            opts: &mesh_opts,
        };

        self.prepare_for_render(gl, &sphere, mesh_name);
        sphere.render(gl, state);

        let mesh_opts = FlatRenderOpts {
            pos: (x + 2., y, z),
            flip_camera_y,
            color: Vector3::new(1., 0., 0.),
            as_lines: false,
        };

        let sphere = Flat {
            object: assets.get_mesh(mesh_name).expect("Sphere"),
            shader,
            opts: &mesh_opts,
        };

        self.prepare_for_render(gl, &sphere, mesh_name);
        sphere.render(gl, state);

        let line_opts = FlatRenderOpts {
            pos: (x, y, z),
            color: Vector3::new(1., 1., 1.),
            flip_camera_y,
            as_lines: true,
        };

        let line_name = "Line";
        let line = Flat {
            object: assets.get_mesh(line_name).expect("Line"),
            shader,
            opts: &line_opts,
        };

        self.prepare_for_render(gl, &line, line_name);
        line.render(gl, state);
    }
}
