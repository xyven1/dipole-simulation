pub(self) use self::mesh::*;
pub(self) use self::render_trait::*;
pub use self::texture_unit::*;
use crate::app::Assets;
use crate::app::State;
use crate::canvas::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::shader::ShaderSystem;
use js_sys::Reflect;
use std::cell::RefCell;
use std::collections::HashMap;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

mod mesh;
mod render_meshes;
mod render_trait;
mod texture_unit;

struct VaoExtension {
    oes_vao_ext: js_sys::Object,
    vaos: RefCell<HashMap<String, Vao>>,
}

struct Vao(js_sys::Object);

pub struct WebRenderer {
    shader_sys: ShaderSystem,
    #[allow(unused)]
    depth_texture_ext: Option<js_sys::Object>,
    vao_ext: VaoExtension,
}

impl WebRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> WebRenderer {
        let shader_sys = ShaderSystem::new(&gl);

        let depth_texture_ext = gl
            .get_extension("WEBGL_depth_texture")
            .expect("Depth texture extension");

        let oes_vao_ext = gl
            .get_extension("OES_vertex_array_object")
            .expect("Get OES vao ext")
            .expect("OES vao ext");

        let vao_ext = VaoExtension {
            oes_vao_ext,
            vaos: RefCell::new(HashMap::new()),
        };

        WebRenderer {
            depth_texture_ext,
            shader_sys,
            vao_ext,
        }
    }

    pub fn render(&mut self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        gl.clear_color(0., 0., 0., 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let above = 1000000.0;
        // Position is positive instead of negative for.. mathematical reasons..
        let clip_plane = [0., 1., 0., above];

        gl.viewport(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);

        self.render_meshes(gl, state, assets, clip_plane, false);
    }

    fn create_vao(&self) -> Vao {
        let oes_vao_ext = &self.vao_ext.oes_vao_ext;

        let create_vao_ext = Reflect::get(oes_vao_ext, &"createVertexArrayOES".into())
            .expect("Create vao func")
            .into();

        Vao(
            Reflect::apply(&create_vao_ext, oes_vao_ext, &js_sys::Array::new())
                .expect("Created vao")
                .into(),
        )
    }

    fn prepare_for_render<'a>(
        &self,
        gl: &WebGlRenderingContext,
        renderable: &impl Render<'a>,
        key: &str,
    ) {
        if self.vao_ext.vaos.borrow().get(key).is_none() {
            let vao = self.create_vao();
            self.bind_vao(&vao);
            renderable.buffer_attributes(gl);
            self.vao_ext.vaos.borrow_mut().insert(key.to_string(), vao);
            return;
        }

        let vaos = self.vao_ext.vaos.borrow();
        let vao = vaos.get(key).unwrap();
        self.bind_vao(vao);
    }

    fn bind_vao(&self, vao: &Vao) {
        let oes_vao_ext = &self.vao_ext.oes_vao_ext;

        let bind_vao_ext = Reflect::get(&oes_vao_ext, &"bindVertexArrayOES".into())
            .expect("Create vao func")
            .into();

        let args = js_sys::Array::new();
        args.push(&vao.0);

        Reflect::apply(&bind_vao_ext, oes_vao_ext, &args).expect("Bound VAO");
    }
}
