use crate::canvas::APP_DIV_ID;
use crate::App;
use crate::Msg;
use nalgebra::Vector3;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;

pub fn append_values(app: Rc<App>) -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let container: HtmlElement = match document.get_element_by_id(APP_DIV_ID) {
        Some(container) => container.dyn_into().expect("Html element"),
        None => document.body().expect("Document body"),
    };

    Ok(())
}

pub fn update_values(app: Rc<App>, l: Vector3<f64>, p: Vector3<f64>, e: f64) {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let angular_momentum = document.get_element_by_id("angular_momentum").unwrap();
    let momentum = document.get_element_by_id("momentum").unwrap();
    let energy = document.get_element_by_id("energy").unwrap();

    angular_momentum.set_inner_html(&format!("({:.5}, {:.5}, {:.5})", l.x, l.y, l.z));
    momentum.set_inner_html(&format!("({:.5}, {:.5}, {:.5})", p.x, p.y, p.z));
    energy.set_inner_html(&format!("{:.5}", e));
}

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
    /* {
        let app = Rc::clone(&app);
        let show_scenery_control = create_show_scenery_control(app)?;
        controls.append_child(&show_scenery_control)?;
    } */
    {
        let app = Rc::clone(&app);
        let time_scale_slider = create_time_scale_control(app)?;
        controls.append_child(&time_scale_slider)?;
    }
    {
        let app = Rc::clone(&app);
        let offset = create_offset_scale_control(app)?;
        controls.append_child(&offset)?;
    }
    {
        let app = Rc::clone(&app);
        let reset = create_reset_button(app)?;
        controls.append_child(&reset)?;
    }
    //append readouts
    let angular_momentum_labal = document.create_element("p")?;
    angular_momentum_labal.set_inner_html("Angular Momentum: ");
    let angular_momentum = document.create_element("span")?;
    angular_momentum.set_id("angular_momentum");
    angular_momentum_labal.append_child(&angular_momentum)?;
    controls.append_child(&angular_momentum_labal)?;

    let momentum = document.create_element("p")?;
    momentum.set_inner_html("Momentum: ");
    let momentum_value = document.create_element("span")?;
    momentum_value.set_id("momentum");
    momentum.append_child(&momentum_value)?;
    controls.append_child(&momentum)?;

    let energy = document.create_element("p")?;
    energy.set_inner_html("Energy: ");
    let energy_value = document.create_element("span")?;
    energy_value.set_id("energy");
    energy.append_child(&energy_value)?;
    controls.append_child(&energy)?;

    Ok(())
}
fn create_reset_button(app: Rc<App>) -> Result<HtmlElement, JsValue> {
    let handler = move |_event: web_sys::Event| {
        app.store.borrow_mut().msg(&Msg::ResetSimulation);
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    let reset_button = Button {
        label: "Reset",
        closure,
    }
    .create_element()?;

    Ok(reset_button)
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

fn create_offset_scale_control(app: Rc<App>) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let offset = input_elem.value_as_number();

        app.store.borrow_mut().msg(&Msg::Offset(offset as f32));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    let offset_slider = Slider {
        start: 1.0,
        min: 0.0,
        max: 10.0,
        step: 0.1,
        label: "Dipole offset",
        closure,
    }
    .create_element()?;

    Ok(offset_slider)
}

struct Button {
    label: &'static str,
    closure: Closure<dyn FnMut(web_sys::Event)>,
}

impl Button {
    fn create_element(self) -> Result<HtmlElement, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let button = document.create_element("button")?;
        let button: HtmlElement = button.dyn_into()?;
        button.set_inner_html(self.label);
        let closure = self.closure;
        button.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        Ok(button)
    }
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
