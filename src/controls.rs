use crate::canvas::APP_DIV_ID;
use crate::App;
use crate::Msg;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;

pub fn append_controls(app: Rc<App>) -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let container: HtmlElement = match document.get_element_by_id(APP_DIV_ID) {
        Some(container) => container.dyn_into().expect("Html element"),
        None => document.body().expect("Document body"),
    };

    let controls = document.create_element("div")?;
    container.append_child(&controls)?;
    let controls: HtmlElement = controls.dyn_into()?;
    controls.style().set_property("padding-left", "5px")?;
    let controls: Element = controls.dyn_into()?;

    // Render Scenery
    {
        let app = Rc::clone(&app);
        let show_scenery_control = create_show_scenery_control(app)?;
        controls.append_child(&show_scenery_control)?;
    }
    {
        let app = Rc::clone(&app);
        let time_scale_slider = create_time_scale_control(app)?;
        controls.append_child(&time_scale_slider)?;
    }
    Ok(())
}

fn create_show_scenery_control(app: Rc<App>) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let show_scenery = input_elem.checked();

        app.store.borrow_mut().msg(&Msg::ShowScenery(show_scenery));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    let show_scenery_control = Checkbox {
        start_checked: true,
        label: "Show Scenery",
        closure,
    }
    .create_element()?;

    Ok(show_scenery_control)
}

fn create_time_scale_control(app: Rc<App>) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let time_scale = input_elem.value_as_number();

        app.store
            .borrow_mut()
            .msg(&Msg::TimeScale(time_scale as f32));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    let time_scale_control = Slider {
        start: 1.0,
        min: 0.0,
        max: 10.0,
        step: 0.1,
        label: "Time Scale",
        closure,
    }
    .create_element()?;

    Ok(time_scale_control)
}

struct Checkbox {
    start_checked: bool,
    label: &'static str,
    closure: Closure<dyn FnMut(web_sys::Event)>,
}

impl Checkbox {
    fn create_element(self) -> Result<HtmlElement, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let checkbox: HtmlInputElement = document.create_element("input")?.dyn_into()?;
        checkbox.set_type("checkbox");
        checkbox.set_checked(self.start_checked);

        let closure = self.closure;
        checkbox.set_oninput(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        let label = document.create_element("label")?;
        label.set_inner_html(self.label);
        label.append_child(&checkbox)?;

        let container = document.create_element("div")?;
        container.append_child(&label)?;

        let container: HtmlElement = container.dyn_into()?;
        container.style().set_property("margin-bottom", "15px")?;
        container.style().set_property("display", "flex")?;
        container.style().set_property("align-items", "center")?;
        container.style().set_property("cursor", "pointer")?;

        Ok(container)
    }
}

struct Slider {
    min: f32,
    max: f32,
    step: f32,
    start: f32,
    label: &'static str,
    closure: Closure<dyn FnMut(web_sys::Event)>,
}

impl Slider {
    fn create_element(self) -> Result<HtmlElement, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let slider: HtmlInputElement = document.create_element("input")?.dyn_into()?;
        slider.set_type("range");
        slider.set_min(&format!("{}", self.min));
        slider.set_max(&format!("{}", self.max));
        slider.set_step(&format!("{}", self.step));
        slider.set_value(&format!("{}", self.start));

        let closure = self.closure;
        slider.set_oninput(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        let label = document.create_element("div")?;
        label.set_inner_html(self.label);

        let container = document.create_element("div")?;
        container.append_child(&label)?;
        container.append_child(&slider)?;

        let container: HtmlElement = container.dyn_into()?;
        container.style().set_property("margin-bottom", "15px")?;

        Ok(container)
    }
}
