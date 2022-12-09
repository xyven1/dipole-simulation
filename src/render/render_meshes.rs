use crate::render::Flat;
use crate::render::FlatRenderOpts;
use crate::render::Render;
use crate::render::WebRenderer;
use crate::shader::ShaderKind;
use crate::simulation::dipole::Object;
use crate::simulation::dipole::Objects;
use crate::Assets;
use crate::State;
use nalgebra::Vector3;
use web_sys::WebGlRenderingContext as GL;

fn render_dipole(
    web: &WebRenderer,
    gl: &GL,
    state: &State,
    assets: &Assets,
    flip_camera_y: bool,
    pos: Vector3<f32>,
    orientation: Vector3<f32>,
    offset: f32,
) {
    // Render Spheres
    let shader = web.shader_sys.get_shader(&ShaderKind::Flat).unwrap();
    web.shader_sys.use_program(gl, ShaderKind::Flat);

    let negative_pos = pos - orientation * offset;
    let mesh_opts = FlatRenderOpts {
        pos: negative_pos,
        orient: Vector3::zeros(),
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

    web.prepare_for_render(gl, &sphere, mesh_name);
    sphere.render(gl, state);

    let positive_pos = pos + orientation * offset;
    let mesh_opts = FlatRenderOpts {
        pos: positive_pos,
        orient: Vector3::zeros(),
        flip_camera_y,
        color: Vector3::new(1., 0., 0.),
        as_lines: false,
    };

    let sphere = Flat {
        object: assets.get_mesh(mesh_name).expect("Sphere"),
        shader,
        opts: &mesh_opts,
    };

    sphere.render(gl, state);

    let line_opts = FlatRenderOpts {
        pos: negative_pos,
        orient: orientation,
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

    web.prepare_for_render(gl, &line, line_name);
    line.render(gl, state);
}

fn render_charge(
    web: &WebRenderer,
    gl: &GL,
    state: &State,
    assets: &Assets,
    flip_camera_y: bool,
    pos: Vector3<f32>,
) {
    let shader = web.shader_sys.get_shader(&ShaderKind::Flat).unwrap();
    web.shader_sys.use_program(gl, ShaderKind::Flat);

    let mesh_opts = FlatRenderOpts {
        pos,
        orient: Vector3::zeros(),
        flip_camera_y,
        color: Vector3::new(0., 1., 0.),
        as_lines: false,
    };
    let mesh_name = "Sphere";

    let sphere = Flat {
        object: assets.get_mesh(mesh_name).expect("Sphere"),
        shader,
        opts: &mesh_opts,
    };

    web.prepare_for_render(gl, &sphere, mesh_name);
    sphere.render(gl, state);
}

impl WebRenderer {
    pub(in crate::render) fn render_meshes(
        &self,
        gl: &GL,
        state: &State,
        assets: &Assets,
        _clip_plane: [f32; 4],
        flip_camera_y: bool,
    ) {
        if !state.show_scenery() {
            return;
        }

        for object in state.simulation.get_objects() {
            match object.get_type() {
                Objects::Dipole => render_dipole(
                    self,
                    gl,
                    state,
                    assets,
                    flip_camera_y,
                    object.get_pos().map(|x| x as f32),
                    object.get_orientation().map(|x| x as f32),
                    object.get_offset() as f32,
                ),
                Objects::Charge => render_charge(
                    self,
                    gl,
                    state,
                    assets,
                    flip_camera_y,
                    object.get_pos().map(|x| x as f32),
                ),
            }
        }
        // render_dipole(
        //     self,
        //     gl,
        //     state,
        //     assets,
        //     flip_camera_y,
        //     state.simulation.get_pos1(),
        // );
        // render_dipole(
        //     self,
        //     gl,
        //     state,
        //     assets,
        //     flip_camera_y,
        //     state.simulation.get_pos2(),
        // );
    }
}
