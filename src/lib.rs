use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use rand::Rng;
extern crate web_sys;


// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }


fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}


fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn canvas() -> web_sys::HtmlCanvasElement {
    let err_msg = "cannot create canvas element";
    document().create_element("canvas").expect(err_msg).dyn_into::<web_sys::HtmlCanvasElement>().expect(err_msg)
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum FlakeType {
    Square,
    Text,
    Circle,
}

#[wasm_bindgen]

pub struct FallingConfig {
    frequency: u8,
    min_radius: f64,
    max_radius: f64,
    min_speed: f64,
    max_speed: f64,
    min_angle: f64,
    max_angle: f64,
    colors: Vec::<String>,
    type_: FlakeType,
    text: String,
    el: String,
}

#[wasm_bindgen]
impl FallingConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(frequency: u8, min_radius: f64, max_radius: f64, min_speed: f64, max_speed: f64, min_angle:f64, max_angle: f64, colors: Vec::<String>, type_: FlakeType, text: String, el: String) -> FallingConfig {
        Self {
            frequency,
            min_radius,
            max_radius,
            min_speed,
            max_speed,
            min_angle,
            max_angle,
            colors,
            type_,
            text,
            el,
        }
    }
}

struct FallingObject {
    x: f64,
    y: f64,
    speed: f64,
    angle: f64,
    color: String,
    radius: f64,
    type_: FlakeType,
    text: String,
    deleted: bool,
}


#[wasm_bindgen]
struct Scene {
    canv: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    config: FallingConfig,
    objects: Vec::<FallingObject>,
    el: web_sys::Element,
    stage_width: f64,
    stage_height: f64,
}

#[wasm_bindgen]
pub fn get_random_f64(fr: f64, to: f64) -> f64 {
    return if fr == to {fr} else {rand::thread_rng().gen_range(fr..to)};
}

#[wasm_bindgen]
pub fn get_random_usize(fr: usize, to: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(fr..to)
}

impl FallingObject {
    pub fn new(scene: &Scene) -> Self {
        Self {
            x: get_random_f64(0f64, scene.stage_width),
            y: 0f64,
            speed: get_random_f64(scene.config.min_speed, scene.config.max_speed),
            angle: get_random_f64(scene.config.min_angle, scene.config.max_angle),
            radius: get_random_f64(scene.config.min_radius, scene.config.max_radius),
            color: scene.config.colors[get_random_usize(0, scene.config.colors.len())].clone(),
            type_: scene.config.type_,
            text: scene.config.text.clone(),
            deleted: false
        }
    }

    pub fn update(&mut self) -> f64 {
        self.x += self.angle * self.speed;
        self.y += self.speed;
        self.y
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        match self.type_ {
            FlakeType::Circle => {
                ctx.set_fill_style(&JsValue::from(&self.color));
                ctx.arc(self.x, self.y, self.radius, 0f64, 2f64 * PI).unwrap();
                ctx.fill();
            }
            FlakeType::Square => {
                ctx.set_fill_style(&JsValue::from(&self.color));
                ctx.fill_rect(self.x,self.y, self.radius * 2f64, self.radius * 2f64);
            }
            FlakeType::Text => {
                ctx.set_font(format!("{}px serif", (self.radius * 5f64) as usize).as_str());
                ctx.set_fill_style(&JsValue::from(&self.color));
                ctx.fill_text(self.text.as_str(), self.x, self.y).unwrap();
            }
        }
        ctx.close_path();
    }
}


#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new(config: FallingConfig) -> Scene {
        let canv = canvas();
        let el = document().query_selector(config.el.as_str()).unwrap().unwrap();
        el.append_child(&canv).unwrap();

        Scene { 
            config,
            ctx: canv.get_context("2d").unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap(),
            canv,
            objects: vec![],
            el,
            stage_height: 0f64,
            stage_width: 0f64,
        }
    }

    pub fn resize(&mut self) {
        self.stage_width = self.el.client_width() as f64;
        self.stage_height = self.el.client_height() as f64;
        self.canv.set_width(self.stage_width as u32);
        self.canv.set_height(self.stage_height as u32);
        // self.canv.set_width((self.stage_width * 2f64) as u32);
        // self.canv.set_height((self.stage_height * 2f64) as u32);
        // self.ctx.scale(2.0, 2.0).unwrap();
    }

    pub fn render(&mut self) {
        self.ctx.clear_rect(0.0, 0.0, self.stage_width as f64, self.stage_height as f64);
        for _ in 0..self.config.frequency {
            self.objects.push(FallingObject::new(self));
        }
        self.objects.retain(|sf| !sf.deleted);
        for i in 0..self.objects.len() {
            if self.objects[i].update() > self.stage_height {
                self.objects[i].deleted = true;
            };
            self.objects[i].render(&self.ctx);
        }
    }
}

