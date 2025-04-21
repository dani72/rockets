use js_sys::Reflect::set_f64;
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlCanvasElement, CanvasRenderingContext2d, KeyboardEvent, HtmlImageElement};
use js_sys::Date;
use web_sys::console;
use std::cell::RefCell;
use std::rc::Rc;

// For better error messages in case of panics
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Game {
    t: i64,
    x: f64,
    y: f64,
    shapes: Vec<Rocket>,

}

trait GameObject {
    fn move_t(&mut self, delta_t: f64);
    fn render(&mut self, ctx: &CanvasRenderingContext2d);
    fn thrust_on( &mut self);
    fn thrust_off( &mut self);
    fn thrust_right( &mut self);
    fn thrust_left( &mut self);
}

struct Vector {
    x: f64,
    y: f64
}

struct Rocket {
    name: String,
    x: f64,
    y: f64,
    rotation: f64,
    speed: Vector,
    acc: Vector,
    thrust: f64,
    sprite_on: Option<HtmlImageElement>,
    sprite_off: Option<HtmlImageElement>
 }

impl Rocket {
    fn status( &mut self) {
        console::log_1( &format!("{}: x = {}, y = {} (Speed {}, {}) (Acc {} {}) (Thrust {})", self.name, self.x, self.y, self.speed.x, self.speed.y, self.acc.x, self.acc.y, self.thrust).into());
    }

    fn update_acc( &mut self) {
        self.acc.x = (self.rotation - std::f32::consts::FRAC_PI_2 as f64).cos() * self.thrust;
        self.acc.y = (self.rotation - std::f32::consts::FRAC_PI_2 as f64).sin() * self.thrust;
    }
}

impl GameObject for Rocket {
    fn move_t(&mut self, delta_t: f64) {

        self.speed.x = self.speed.x + (self.acc.x * delta_t);
        self.speed.y = self.speed.y + ((self.acc.y + 9.81) * delta_t);

        self.x += self.speed.x * delta_t;
        self.y += self.speed.y * delta_t;

    }

    fn render(&mut self, ctx: &CanvasRenderingContext2d) {

        if self.thrust > 0.0 {
            if let Some(sprite) = &self.sprite_on {
                ctx.save();
                ctx.translate(self.x, self.y).unwrap();          // Move to sprite position
                ctx.rotate( self.rotation).unwrap();        // Rotate around that point
                ctx.draw_image_with_html_image_element_and_dw_and_dh(
                    sprite,
                    - (sprite.width() as f64 / 2.0),
                    - (sprite.height() as f64 / 2.0),
                    sprite.width() as f64,
                    sprite.height() as f64,
                ).unwrap();
                ctx.restore();
            }
        }
        else {
            if let Some(sprite) = &self.sprite_off {
                ctx.save();
                ctx.translate(self.x, self.y).unwrap();          // Move to sprite position
                ctx.rotate( self.rotation).unwrap();        // Rotate around that point
                ctx.draw_image_with_html_image_element_and_dw_and_dh(
                    sprite,
                    - (sprite.width() as f64 / 2.0),
                    - (sprite.height() as f64 / 2.0),
                    sprite.width() as f64,
                    sprite.height() as f64,
                ).unwrap();
                ctx.restore();
            }

        }
    }

    fn thrust_off( &mut self) {
        if self.thrust > 0.0 {
            self.thrust -= 1.0;
        }

        self.update_acc();
        self.status();
    }

    fn thrust_on( &mut self) {
        if self.thrust < 20.0 {
            self.thrust += 1.0;
        }

        self.update_acc();
        self.status();
    }

    fn thrust_right( &mut self) {
        self.rotation += 0.1;

        self.update_acc();
        self.status();
    }

    fn thrust_left( &mut self) {
        self.rotation -= 0.1;

        self.update_acc();
        self.status();
    }

}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        let circle = Rocket {
            name: "Rocket1".to_string(),
            x: 480.0,
            y: 100.0,
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
        let circle2 = Rocket {
            name: "Rocket2".to_string(),
            x: 300.0,
            y: 50.0,
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
            x: 200.0,
            y: 200.0,
            shapes: vec![circle, circle2],
            sprite_on: None,
            sprite_off: None
        }
    }

    pub fn set_rocket_thrust_on(&mut self, image: HtmlImageElement) {
//        self.shapes[0].sprite_on = Some(image);

        let document = window().unwrap().document().unwrap();

        let img1 = document.create_element("img").unwrap().dyn_into::<HtmlImageElement>().unwrap();
        let img2 = document.create_element("img").unwrap().dyn_into::<HtmlImageElement>().unwrap();

        img1.set_src( &image.src());
        img2.set_src( &image.src());

        self.shapes[0].sprite_on = Some(img1);
        self.shapes[1].sprite_on = Some(img2);
    }

    pub fn set_rocket_thrust_off(&mut self, image: HtmlImageElement) {
//        self.shapes[0].sprite_off = Some(image);

        let document = window().unwrap().document().unwrap();

        let img1 = document.create_element("img").unwrap().dyn_into::<HtmlImageElement>().unwrap();
        let img2 = document.create_element("img").unwrap().dyn_into::<HtmlImageElement>().unwrap();

        img1.set_src( &image.src());
        img2.set_src( &image.src());

        self.shapes[0].sprite_off = Some(img1);
        self.shapes[1].sprite_off = Some(img2);
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
        self.shapes[0].thrust_on();
    }

    pub fn down_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("down pressed"));
        self.shapes[0].thrust_off();
    }

    pub fn left_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("left pressed"));
        self.shapes[0].thrust_left();
    }

    pub fn right_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("right pressed"));
        self.shapes[0].thrust_right();
    }

    pub fn a_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("left pressed"));
        self.shapes[1].thrust_left();
    }

    pub fn d_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("right pressed"));
        self.shapes[1].thrust_right();
    }

    pub fn w_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("up pressed"));
        self.shapes[1].thrust_on();
    }

    pub fn x_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("down pressed"));
        self.shapes[1].thrust_off();
    }
}
