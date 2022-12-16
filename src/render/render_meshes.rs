use crate::render::Flat;
use crate::render::FlatRenderOpts;
use crate::render::Render;
use crate::render::WebRenderer;
use crate::shader::ShaderKind;
use crate::simulation::dipole::Object;
use crate::simulation::dipole::Objects;
use crate::webgl_object::WebGLObject;
use crate::Assets;
use crate::State;
use nalgebra::Vector3;
use web_sys::WebGlRenderingContext as GL;

fn render_field_lines(
    web: &WebRenderer,
    gl: &GL,
    state: &State,
    assets: &Assets,
    flip_camera_y: bool,
    start_pos: Vector3<f64>,
) {
    let shader = web.shader_sys.get_shader(&ShaderKind::Flat).unwrap();
    web.shader_sys.use_program(gl, ShaderKind::Flat);

    let line_opts = FlatRenderOpts {
        pos: Vector3::new(0.0, 0.0, 0.0),
        orient: Vector3::new(0.0, 0.0, 0.0),
        color: Vector3::new(1., 0., 1.),
        flip_camera_y,
        as_lines: true,
    };

    let mut field_lines: Vec<Vector3<f32>> = Vec::new();

    field_lines.push(start_pos.cast());
    for i in 0..1 {
        let field = state
            .simulation
            .get_field(field_lines[i].cast())
            .normalize()
            * 1.0;
        field_lines.push(Vector3::new(
            field_lines[i].x + field.x as f32,
            field_lines[i].y + field.y as f32,
            field_lines[i].z + field.z as f32,
        ));
    }

    let object = WebGLObject {
        vertices: field_lines
            .iter()
            .flat_map(|v| v.as_slice())
            .copied()
            .collect(),
        indices: vec![],
        normals: vec![],
    };
    // web_sys::console::log_1(&format!("field lines: {:?}", object.vertices).into());

    let line = Flat {
        object: &object,
        shader,
        opts: &line_opts,
    };

    // line.buffer_attributes(gl);
    web.prepare_for_render(gl, &line, "FieldLines");
    line.buffer_attributes(gl);
    line.render(gl, state);
}

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
        pos,
        orient: orientation,
        color: Vector3::new(1., 1., 1.),
        flip_camera_y,
        as_lines: true,
    };

    let line_name = format!("Line{}", offset);
    let object = Assets::gen_line(Vector3::new(-offset, 0., 0.), Vector3::new(offset, 0., 0.));
    let line = Flat {
        object: &object,
        shader,
        opts: &line_opts,
    };

    web.prepare_for_render(gl, &line, line_name.as_str());
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

fn render_axis(web: &WebRenderer, gl: &GL, state: &State, assets: &Assets, flip_camera_y: bool) {
    let shader = web.shader_sys.get_shader(&ShaderKind::Flat).unwrap();
    web.shader_sys.use_program(gl, ShaderKind::Flat);

    let line_opts = FlatRenderOpts {
        pos: Vector3::zeros(),
        orient: Vector3::zeros(),
        color: Vector3::new(1., 0., 0.),
        flip_camera_y,
        as_lines: true,
    };

    let line_name = "LineX";
    let object = Assets::gen_line(Vector3::new(-10., 0., 0.), Vector3::new(10., 0., 0.));
    let line = Flat {
        object: &object,
        shader,
        opts: &line_opts,
    };

    web.prepare_for_render(gl, &line, line_name);
    line.render(gl, state);

    let line_opts = FlatRenderOpts {
        pos: Vector3::zeros(),
        orient: Vector3::zeros(),
        color: Vector3::new(0., 1., 0.),
        flip_camera_y,
        as_lines: true,
    };

    let line_name = "LineY";
    let object = Assets::gen_line(Vector3::new(0., -10., 0.), Vector3::new(0., 10., 0.));
    let line = Flat {
        object: &object,
        shader,
        opts: &line_opts,
    };

    web.prepare_for_render(gl, &line, line_name);
    line.render(gl, state);

    let line_opts = FlatRenderOpts {
        pos: Vector3::zeros(),
        orient: Vector3::zeros(),
        color: Vector3::new(0., 0., 1.),
        flip_camera_y,
        as_lines: true,
    };

    let line_name = "LineZ";
    let object = Assets::gen_line(Vector3::new(0., 0., -10.), Vector3::new(0., 0., 10.));
    let line = Flat {
        object: &object,
        shader,
        opts: &line_opts,
    };

    web.prepare_for_render(gl, &line, line_name);
    line.render(gl, state);
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

        /* for i in (-5..5).step_by(3) {
            for j in (-5..5).step_by(3) {
                for k in (-5..5).step_by(3) {
                    render_field_lines(self, gl, state, assets, flip_camera_y, Vector3::new(i as f64, j as f64, k as f64));
                }
            }
        } */

        render_axis(self, gl, state, assets, flip_camera_y);

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
