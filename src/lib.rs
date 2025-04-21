mod vmath;
mod rocket;
mod asteroid;
mod game;
mod myrand;

use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlImageElement};
use js_sys::Date;
use vmath::Vector;
use rocket::Rocket;
use asteroid::Asteroid;
use game::GameObject;
use game::GameArea;
use game::Area;
use myrand::random_number;

pub fn clone_sprite( image: &HtmlImageElement) -> HtmlImageElement{
    let document = window().unwrap().document().unwrap();
    let img1 = document.create_element("img").unwrap().dyn_into::<HtmlImageElement>().unwrap();
    img1.set_src( &image.src());

    return img1;
}

impl GameArea for Game {
    fn area( &self) -> Area {
        self.game_area.clone()
    }
}

// For better error messages in case of panics
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Game {
    game_area: Area,
    ast : HtmlImageElement,
    ctx: CanvasRenderingContext2d,
    t: i64,
    shapes: Vec<Box<dyn GameObject>>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new( asteroid_sprite: HtmlImageElement, rocket_thrust_on: HtmlImageElement, rocket_thrust_off: HtmlImageElement, rendering_context: CanvasRenderingContext2d ) -> Game {
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
            sprite_on: clone_sprite( &rocket_thrust_on),
            sprite_off: clone_sprite( &rocket_thrust_off)
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
            sprite_on: clone_sprite( &rocket_thrust_on),
            sprite_off: clone_sprite( &rocket_thrust_off)
        };
        Game {
            game_area: Area {  // Game area
                width: 1000.0,
                height: 600.0,
            },
            ctx: rendering_context,
            ast: asteroid_sprite,
            t: Self::now_ms(),
            shapes: vec![
                Box::new( rocket1),
                Box::new( rocket2)]
        }
    }

    pub fn add_asteroids( &mut self) {
        for i in 0..10 {
            let asteroid = Asteroid {
                name: "Asteroid".to_string(),
                position: Vector {
                    x: 20.0 * random_number(),
                    y: 15.0 * random_number()
                },
                rotation: 0.0,
                speed: Vector {
                    x: 5.0 + random_number(),
                    y: 5.0 - random_number()
                },
                acc : Vector {
                    x: 0.0,
                    y: 0.0
                },
                image: clone_sprite( &self.ast)
            };

            self.shapes.push( Box::new( asteroid));
        }
    }

    pub fn now_ms() -> i64 {
        Date::now() as i64
    }

    pub fn update(&mut self) {
        let delta_t = Self::now_ms() - self.t;

        for shape in self.shapes.iter_mut() {
            shape.move_t( delta_t as f64 / 1000.0, self.game_area.clone());
        }

        self.t = Self::now_ms();
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        self.update();
        
        self.ctx.clear_rect(0.0, 0.0, self.game_area.width, self.game_area.height);
        
        for shape in self.shapes.iter_mut() {
            shape.render( &self.ctx);
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
