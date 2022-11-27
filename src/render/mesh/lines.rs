use crate::app::State;
use crate::render::Render;
use crate::shader::Shader;
use crate::shader::ShaderKind;
use nalgebra;
use nalgebra::{Isometry3, Vector3};
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Line<'a> {
    pub vertices: &'a [f32],
    pub shader: &'a Shader,
    pub opts: &'a LineRenderOpts,
}

pub struct LineRenderOpts {
    pub pos: (f32, f32, f32),
    pub clip_plane: [f32; 4],
    pub flip_camera_y: bool,
}

impl<'a> Render<'a> for Line<'a> {
    fn shader_kind() -> ShaderKind {
        ShaderKind::Line
    }

    fn shader(&'a self) -> &'a Shader {
        &self.shader
    }

    fn buffer_attributes(&self, gl: &WebGlRenderingContext) {
        let shader = self.shader();
        let vertices = self.vertices;

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");

        gl.enable_vertex_attrib_array(pos_attrib as u32);

        Line::buffer_f32_data(&gl, &vertices, pos_attrib as u32, 3);
    }

    fn render(&self, gl: &WebGlRenderingContext, state: &State) {
        let shader = self.shader();

        let vertices = self.vertices;
        let opts = self.opts;
        let pos = opts.pos;

        let model_uni = shader.get_uniform_location(gl, "model");
        let view_uni = shader.get_uniform_location(gl, "view");
        let perspective_uni = shader.get_uniform_location(gl, "perspective");

        let mut view = if opts.flip_camera_y {
            state.camera().view_flipped_y()
        } else {
            state.camera().view()
        };
        gl.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &mut view);

        let model = Isometry3::new(Vector3::new(pos.0, pos.1, pos.2), nalgebra::zero());
        let mut model_array = [0.; 16];
        model_array.copy_from_slice(model.to_homogeneous().as_slice());
        gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);

        let mut perspective = state.camera().projection();
        gl.uniform_matrix4fv_with_f32_array(perspective_uni.as_ref(), false, &mut perspective);

        let num_vertices = vertices.len() / 3;
        gl.draw_arrays(GL::LINE_STRIP, 0, num_vertices as i32);
    }
}
