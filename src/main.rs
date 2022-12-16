use std::cell::Cell;
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

#[derive(PartialEq, Debug)]
pub struct SimulationState {
    time_scale: f64,
    current_time: f64,
}

type CanvasCallback = Callback<Rc<Option<WebGlRenderingContext>>>;

#[derive(Properties, PartialEq)]
pub struct CanvasProps {
    pub on_init: CanvasCallback,
    pub on_render: CanvasCallback,
}

pub struct Canvas {
    node_ref: NodeRef,
    gl: Rc<Option<WebGlRenderingContext>>,
    on_render: CanvasCallback,
}

impl Component for Canvas {
    type Message = ();
    type Properties = CanvasProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            gl: Rc::new(None),
            on_render: ctx.props().on_render.clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
        ctx.props().on_init.emit(self.gl.clone());
        self.render();
    }
}

impl Canvas {
    fn render(&self) {
        let cb = Rc::new(RefCell::new(None));

        let gl = self.gl.clone();
        let render_cb = self.on_render.clone();

        *cb.borrow_mut() = Some(Closure::wrap(Box::new({
            let cb = cb.clone();
            move || {
                render_cb.emit(gl.clone());
                /* if let Some(renderer) = &renderer {
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
                } */
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

pub struct MainAppReal {
    time_scale: f64,
    current_time: f64,
    renderer: Rc<RefCell<Option<WebRenderer>>>,
    gl: Rc<RefCell<Rc<Option<WebGlRenderingContext>>>>,
    assets: Rc<Assets>,
    state: Rc<RefCell<Store>>,
    on_init: CanvasCallback,
    on_render: CanvasCallback,
}

impl Component for MainAppReal {
    type Properties = ();
    type Message = ();

    fn create(ctx: &Context<Self>) -> Self {
        let gl = Rc::new(RefCell::new(Rc::new(None)));
        let renderer = Rc::new(RefCell::new(None));
        let assets = Rc::new(Assets::new());
        let store = Rc::new(RefCell::new(Store::new()));

        let on_init = {
            let app_gl = gl.clone();
            let app_renderer = renderer.clone();
            Callback::from(move |gl: Rc<Option<WebGlRenderingContext>>| {
                web_sys::console::log_1(&"on_init".into());
                *app_gl.borrow_mut() = gl.clone();
                *app_renderer.borrow_mut() = Some(WebRenderer::new(gl.as_ref().as_ref().unwrap()));
            })
        };
        let on_render = {
            let app_renderer = renderer.clone();
            let app_assets = assets.clone();
            let app_store = store.clone();
            Callback::from(move |gl: Rc<Option<WebGlRenderingContext>>| {
                if let Some(renderer) = &*app_renderer.borrow() {
                    app_store.borrow_mut().msg(&Msg::UpdateSimulation(1.0));
                    renderer.render(
                        gl.as_ref().as_ref().unwrap(),
                        &app_store.borrow().state,
                        app_assets.as_ref(),
                    );
                }
            })
        };
        Self {
            time_scale: 1.0,
            current_time: 0.0,
            renderer,
            gl,
            on_init,
            on_render,
            assets,
            state: store,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = self.state.clone();
        let onclick = ctx.link().callback(move |_| {
            state.borrow_mut().msg(&Msg::UpdateSimulation(1.0));
        });
        html! {
            <div>
                <Canvas on_init={self.on_init.clone()} on_render={self.on_render.clone()} />
                <button onclick={onclick}>{"Update"}</button>
            </div>
        }
    }
}

// #[function_component]
/* fn App() -> Html {
    let simulation = use_state_eq(|| Simulation {
        time_scale: 1.0,
        current_time: 0.0,
    });
    let sim = simulation.clone();

    let renderer: Rc<RefCell<WebRenderer>>;
    let on_init: CanvasCallback = Callback::from(move |gl: Rc<Option<WebGlRenderingContext>>| {
        let gl = gl.as_ref().as_ref().unwrap();
        //init app
        app = Rc::new(RefCell::new(app::App::new()));
        //init renderer
        renderer = Rc::new(RefCell::new(WebRenderer::new(gl)));
    });

    let on_render: CanvasCallback = Callback::from(move |gl: Rc<Option<WebGlRenderingContext>>| {
        if let Some(gl) = gl.as_ref() {
            gl.clear_color(
                (sim.current_time / 100.0).clamp(0.0, 1.0) as f32,
                rand::thread_rng().gen_range(0.0..1.0) as f32,
                0.0,
                1.0,
            );
            gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        }
    });

    let sim0 = simulation.clone();
    html! {
        <div>
            <Canvas on_render={on_render} on_init={on_init} />
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
} */

fn main() {
    yew::Renderer::<MainAppReal>::new().render();
}
