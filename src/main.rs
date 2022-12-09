use std::{cell::RefCell, rc::Rc};

use rand::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};
use yew::prelude::*;

mod app;
mod generate_sphere;
mod render;
mod shader;
mod simulation;
mod webgl_object;
use crate::app::*;
use crate::render::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Simulation {
    time_scale: f64,
    current_time: f64,
}

#[derive(Properties, PartialEq)]
pub struct CanvasProps {
    pub simulation: Simulation,
    pub on_render: Callback<(Rc<Option<WebGlRenderingContext>>, Simulation)>,
}

pub struct Canvas {
    node_ref: NodeRef,
    gl: Rc<Option<WebGlRenderingContext>>,
    simulation: Rc<RefCell<Simulation>>,
    on_render: Callback<(Rc<Option<WebGlRenderingContext>>, Simulation)>,
}

impl Component for Canvas {
    type Message = ();
    type Properties = CanvasProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            gl: Rc::new(None),
            simulation: Rc::new(RefCell::new(ctx.props().simulation)),
            on_render: ctx.props().on_render.clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.simulation
            .borrow_mut()
            .clone_from(&ctx.props().simulation);
        html! {
            <canvas width="512" height="512" ref={self.node_ref.clone()} />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        let canvas: HtmlCanvasElement = self.node_ref.cast().unwrap();
        let gl = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();
        self.gl = Rc::new(Some(gl));
        self.render();
    }
}

impl Canvas {
    fn render(&self) {
        let cb = Rc::new(RefCell::new(None));

        let gl = self.gl.clone();
        let render_cb = self.on_render.clone();
        let simulation = self.simulation.clone();

        let renderer = gl.as_ref().as_ref().map(WebRenderer::new);
        let assets = Rc::new(Assets::new());
        let store = Rc::new(RefCell::new(Store::new()));

        *cb.borrow_mut() = Some(Closure::wrap(Box::new({
            let cb = cb.clone();
            move || {
                /* if let Ok(simulation) = simulation.try_borrow() {
                    render_cb.emit((gl.clone(), *simulation));
                    web_sys::console::log_1(&format!("sim: {:?}", simulation).into());
                } */
                render_cb.emit((gl.clone(), *simulation.borrow()));
                if let Some(renderer) = &renderer {
                    store.borrow_mut().msg(&Msg::UpdateSimulation(1.0));
                    let l = store.borrow().state.simulation.get_total_angular_momentum();
                    let p = store.borrow().state.simulation.get_total_momentum();
                    let e = store.borrow().state.simulation.get_total_energy();
                    web_sys::console::log_1(&format!("L: {:?}, P: {:?}, E: {:?}", l, p, e).into());

                    renderer.render(
                        gl.as_ref().as_ref().unwrap(),
                        &store.borrow().state,
                        assets.as_ref(),
                    );
                }
                Canvas::request_animation_frame(cb.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>));

        Canvas::request_animation_frame(cb.borrow().as_ref().unwrap());
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        web_sys::window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }
}

#[function_component]
fn App() -> Html {
    let simulation = use_state_eq(|| Simulation {
        time_scale: 1.0,
        current_time: 0.0,
    });

    let on_render = Callback::from(
        move |(gl, sim): (Rc<Option<WebGlRenderingContext>>, Simulation)| {
            if let Some(gl) = gl.as_ref() {
                gl.clear_color(
                    (sim.current_time / 100.0).clamp(0.0, 1.0) as f32,
                    rand::thread_rng().gen_range(0.0..1.0) as f32,
                    0.0,
                    1.0,
                );
                gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
            }
        },
    );

    let sim0 = simulation.clone();
    html! {
        <div>
            <Canvas simulation={*sim0} on_render={on_render.clone()} />
                <button onclick={Callback::from(move |_| {
                    sim0.set(Simulation{
                        time_scale: 1.0,
                        current_time: sim0.current_time + 10.0,
                    });
                })}>
                    {"+"}
                </button>
            <p>{ "Hello, world!" }</p>
            <p>{ "The current time is: "} {simulation.current_time}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
