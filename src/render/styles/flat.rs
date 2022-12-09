use crate::app::State;
use crate::render::Render;
use crate::shader::Shader;
use crate::shader::ShaderKind;
use crate::webgl_object::WebGLObject;
use nalgebra::Matrix4;
use nalgebra::Rotation3;
use nalgebra::{Isometry3, Vector3};
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Flat<'a> {
    pub object: &'a WebGLObject,
    pub shader: &'a Shader,
    pub opts: &'a FlatRenderOpts,
}

pub struct FlatRenderOpts {
    pub color: Vector3<f32>,
    pub pos: Vector3<f32>,
    pub orient: Vector3<f32>,
    pub as_lines: bool,
    pub flip_camera_y: bool,
}

impl<'a> Render<'a> for Flat<'a> {
    fn shader_kind() -> ShaderKind {
        ShaderKind::Flat
    }

    fn shader(&'a self) -> &'a Shader {
        self.shader
    }

    fn buffer_attributes(&self, gl: &WebGlRenderingContext) {
        let shader = self.shader();

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");

        gl.enable_vertex_attrib_array(pos_attrib as u32);

        Flat::buffer_f32_data(gl, &self.object.vertices, pos_attrib as u32, 3);
        if !self.opts.as_lines {
            Flat::buffer_u16_indices(gl, &self.object.indices[..]);
        }
    }

    fn render(&self, gl: &WebGlRenderingContext, state: &State) {
        let shader = self.shader();

        let opts = self.opts;
        let pos = opts.pos;

        let model_uni = shader.get_uniform_location(gl, "model");
        let view_uni = shader.get_uniform_location(gl, "view");
        let perspective_uni = shader.get_uniform_location(gl, "perspective");
        let color_uni = shader.get_uniform_location(gl, "uColor");

        let mut view = if opts.flip_camera_y {
            state.camera().view_flipped_y()
        } else {
            state.camera().view()
        };
        gl.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &mut view);

        let mut model = Matrix4::new_translation(&Vector3::new(pos.x, pos.y, pos.z));
        if let Some(rotation) = Rotation3::rotation_between(&Vector3::new(1., 0., 0.), &opts.orient) {
            model *= rotation.to_homogeneous();
        }
        let mut model_array = [0.; 16];
        model_array.copy_from_slice(model.as_slice());
        gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);

        let mut perspective = state.camera().projection();
        gl.uniform_matrix4fv_with_f32_array(perspective_uni.as_ref(), false, &mut perspective);

        gl.uniform3f(color_uni.as_ref(), opts.color.x, opts.color.y, opts.color.z);

        if self.opts.as_lines {
            let num_vertices = self.object.vertices.len() / 3;
            gl.draw_arrays(GL::LINE_STRIP, 0, num_vertices as i32);
        } else {
            let num_indices = self.object.indices.len();
            gl.draw_elements_with_i32(GL::TRIANGLES, num_indices as i32, GL::UNSIGNED_SHORT, 0);
        }
    }
}
