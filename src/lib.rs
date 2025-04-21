mod vmath;
mod rocket;
mod game;

use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d, HtmlImageElement};
use js_sys::Date;
use vmath::Vector;
use rocket::Rocket;
use game::GameObject;

// For better error messages in case of panics
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Game {
    t: i64,
    shapes: Vec<Box<dyn GameObject>>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        let rocket1 = Rocket {
            name: "Rocket1".to_string(),
            position: Vector {
                x: 480.0,
                y: 100.0
            },
            rotation: 0.0,
            speed: Vector {
                x: 0.0,
                y: 0.0
            },
            acc : Vector {
                x: 0.0,
                y: 0.0
            },
            thrust: 0.0,
            sprite_on: None,
            sprite_off: None
        };
        let rocket2 = Rocket {
            name: "Rocket2".to_string(),
            position: Vector {
                x: 300.0,
                y: 50.0,
            },
            rotation: 0.0,
            speed: Vector {
                x: 0.0,
                y: 0.0
            },
            acc : Vector {
                x: 0.0,
                y: 0.0
            },
            thrust: 0.0,
            sprite_on: None,
            sprite_off: None
        };
        Game {
            t: Self::now_ms(),
            shapes: vec![
                Box::new( rocket1),
                Box::new( rocket2)]
        }
    }

    pub fn set_rocket_thrust_on(&mut self, image: HtmlImageElement) {
        let document = window().unwrap().document().unwrap();

        let img1 = document.create_element("img").unwrap().dyn_into::<HtmlImageElement>().unwrap();
        let img2 = document.create_element("img").unwrap().dyn_into::<HtmlImageElement>().unwrap();

        img1.set_src( &image.src());
        img2.set_src( &image.src());

        self.shapes[0].set_sprite_on( img1);
        self.shapes[1].set_sprite_on( img2);
    }

    pub fn set_rocket_thrust_off(&mut self, image: HtmlImageElement) {
        let document = window().unwrap().document().unwrap();

        let img1 = document.create_element("img").unwrap().dyn_into::<HtmlImageElement>().unwrap();
        let img2 = document.create_element("img").unwrap().dyn_into::<HtmlImageElement>().unwrap();

        img1.set_src( &image.src());
        img2.set_src( &image.src());

        self.shapes[0].set_sprite_off (img1);
        self.shapes[1].set_sprite_off (img2);
    }


    pub fn now_ms() -> i64 {
        Date::now() as i64
    }

    pub fn update(&mut self) {
        let delta_t = Self::now_ms() - self.t;

        for shape in self.shapes.iter_mut() {
            shape.move_t( delta_t as f64 / 1000.0);
        }

        self.t = Self::now_ms();
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        // Get the document
        let document = window()
            .ok_or_else(|| JsValue::from_str("No global window exists"))?
            .document()
            .ok_or_else(|| JsValue::from_str("No document exists"))?;
        
        // Get the canvas
        let canvas = document
            .get_element_by_id("game-canvas")
            .ok_or_else(|| JsValue::from_str("No canvas found with id 'game-canvas'"))?;
        
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| JsValue::from_str("Element is not a canvas"))?;
        
        // Get the context
        let context = canvas
            .get_context("2d")
            .map_err(|_| JsValue::from_str("Failed to get 2d context"))?
            .ok_or_else(|| JsValue::from_str("No 2d context found"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| JsValue::from_str("Context is not a 2d context"))?;
        
        // Update game state
        self.update();
        
        // Clear canvas
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        
        // Draw circle
        context.begin_path();

        for shape in self.shapes.iter_mut() {
            shape.render( &context);
        }
        
        Ok(())
    }

    pub fn up_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("up pressed"));
        self.shapes[0].thrust_inc();
    }

    pub fn down_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("down pressed"));
        self.shapes[0].thrust_dec();
    }

    pub fn left_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("left pressed"));
        self.shapes[0].rotate_left();
    }

    pub fn right_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("right pressed"));
        self.shapes[0].rotate_right();
    }

    pub fn a_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("left pressed"));
        self.shapes[1].rotate_left();
    }

    pub fn d_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("right pressed"));
        self.shapes[1].rotate_right();
    }

    pub fn w_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("up pressed"));
        self.shapes[1].thrust_inc();
    }

    pub fn x_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("down pressed"));
        self.shapes[1].thrust_dec();
    }
}
