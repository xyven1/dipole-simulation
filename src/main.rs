use std::rc::Rc;
mod app;
mod canvas;
mod controls;
mod generate_sphere;
mod render;
mod shader;
mod simulation;
mod webgl_object;

use app::App;
use render::WebRenderer;
use canvas::Canvas;
use yew::prelude::*;

#[function_component]
fn AppFunc() -> Html {
    //get element by id
    let app = use_state(|| {
        App::new()
    });
    let renderer = use_state(|| {
        WebRenderer::new()
    });


    html! {
        <div>
            <Canvas id="canvas" width="800" height="600"
                onmousedown={on_mouse_down}
                onmouseup={on_mouse_up}
                onmousemove={on_mouse_move}
                onwheel={on_mouse_wheel}/>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<AppFunc>::new().render();
}
