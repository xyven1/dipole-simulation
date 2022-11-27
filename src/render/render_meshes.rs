use crate::render::LineRenderOpts;
use crate::render::MeshRenderOpts;
use crate::render::Render;
use crate::render::Mesh;
use crate::render::Line;
use crate::render::WebRenderer;
use crate::shader::ShaderKind;
use crate::Assets;
use crate::State;
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

        // Render Terrain

        let shader = self.shader_sys.get_shader(&ShaderKind::Mesh).unwrap();
        self.shader_sys.use_program(gl, ShaderKind::Mesh);

        let mesh_opts = MeshRenderOpts {
            pos: (0., 0., 0.),
            clip_plane,
            flip_camera_y,
        };

        let mesh_name = "Terrain";
        let terrain = Mesh {
            mesh: assets.get_mesh(mesh_name).expect("Terrain mesh"),
            shader,
            opts: &mesh_opts,
        };

        self.prepare_for_render(gl, &terrain, mesh_name);
        terrain.render(gl, state);

        //render line
        let shader = self.shader_sys.get_shader(&ShaderKind::Line).unwrap();
        self.shader_sys.use_program(gl, ShaderKind::Line);

        let line_opts = LineRenderOpts {
            pos: (0., 0., 0.),
            clip_plane,
            flip_camera_y,
        };

        let line_name = "Line";
        let line = Line {
            vertices: &[0., 0., 0., 1., 1., 1.],
            shader,
            opts: &line_opts,
        };

        self.prepare_for_render(gl, &line, line_name);
        line.render(gl, state);
    }
}
