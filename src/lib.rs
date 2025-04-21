mod vmath;
mod rocket;
mod asteroid;
mod game;
mod myrand;
mod explosion;
mod bullet;

use game::GameObjectType;
use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlImageElement};
use js_sys::Date;
use vmath::Vector;
use rocket::Rocket;
use asteroid::Asteroid;
use explosion::Explosion;
use game::GameObject;
use game::ActiveObject;
use game::Area;
use myrand::random_number;
use vmath::ZERO;

pub fn clone_sprite( image: &HtmlImageElement) -> HtmlImageElement{
    let document = window().unwrap().document().unwrap();
    let img1 = document.create_element("img").unwrap().dyn_into::<HtmlImageElement>().unwrap();
    img1.set_src( &image.src());

    return img1;
}

// For better error messages in case of panics
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Game {
    rocket1: Rocket,
    rocket2: Rocket,
    game_area: Area,
    ast : HtmlImageElement,
    exp : HtmlImageElement,
    ctx: CanvasRenderingContext2d,
    t: i64,
    bullets: Vec<Box<dyn GameObject>>,
    shapes: Vec<Box<dyn GameObject>>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new( game_width: f64,
                game_height: f64,
                asteroid_sprite: HtmlImageElement, 
                rocket_thrust_on: HtmlImageElement, 
                rocket_thrust_off: HtmlImageElement,
                explosion_sprite: HtmlImageElement,
                rendering_context: CanvasRenderingContext2d ) -> Game {
        Game {
            rocket1: Rocket {
                name: "Rocket1".to_string(),
                position: Vector { x: 480.0, y: 100.0 },
                rotation: 0.0,
                speed: ZERO,
                acc : ZERO,
                thrust: 0.0,
                sprite_on: clone_sprite( &rocket_thrust_on),
                sprite_off: clone_sprite( &rocket_thrust_off)
            },
            rocket2: Rocket {
                name: "Rocket2".to_string(),
                position: Vector { x: 300.0, y: 50.0 },
                rotation: 0.0,
                speed: ZERO,
                acc : ZERO,
                thrust: 0.0,
                sprite_on: clone_sprite( &rocket_thrust_on),
                sprite_off: clone_sprite( &rocket_thrust_off)
            },
            game_area: Area {  // Game area
                width: game_width,
                height: game_height,
            },
            ctx: rendering_context,
            ast: asteroid_sprite,
            exp: explosion_sprite,
            t: Self::now_ms(),
            bullets: vec![],
            shapes: vec![]
        }
    }

    pub fn create_asteroids( &mut self, nof : i32) {
        let mut i = 0;

        while i < nof  {
            let asteroid = Asteroid {
                expired: false,
                position: Vector { x: 20.0 * random_number(), y: 15.0 * random_number() },
                rotation: 0.0,
                speed: Vector { x: 5.0 + random_number(), y: 5.0 - random_number() },
                acc : ZERO,
                image: clone_sprite( &self.ast)
            };

            self.shapes.push( Box::new( asteroid));

            i+=1;
        }
    }

    pub fn now_ms() -> i64 {
        Date::now() as i64
    }

    fn update(&mut self) {
        let delta_t = (Self::now_ms() - self.t) as f64 / 1000.0;
        
        self.rocket1.move_t( delta_t, self.game_area.clone());
        self.rocket2.move_t( delta_t, self.game_area.clone());

        for bullet in self.bullets.iter_mut() {
            bullet.move_t( delta_t, self.game_area.clone());
        }

        for shape in self.shapes.iter_mut() {
            shape.move_t( delta_t, self.game_area.clone());
        }

        self.t = Self::now_ms();
    }

    fn check_collisions(&mut self) {
        let mut expl: Option<Explosion> = None;

        for shape in self.shapes.iter_mut() {
            if self.rocket1.current_position().distance( &shape.current_position()) < 30.0 {
                if matches!( shape.get_type(), GameObjectType::Asteroid) {
                    shape.expire();
                    expl = Some( Explosion {
                        time: 0.0,
                        position: shape.current_position(),
                        image: clone_sprite( &self.exp)
                    });
                }
            }

            if self.rocket2.current_position().distance( &shape.current_position()) < 30.0 {
                if matches!( shape.get_type(), GameObjectType::Asteroid) {
                    shape.expire();
                    expl = Some( Explosion {
                        time: 0.0,
                        position: shape.current_position(),
                        image: clone_sprite( &self.exp)
                    });
                }
            }
        }

        if let Some( explosion) = expl {
            self.shapes.push( Box::new( explosion));
        }

        let mut expl2: Option<Explosion> = None;

        for bullet in self.bullets.iter_mut() {
            for shape in self.shapes.iter_mut() {
                if bullet.current_position().distance( &shape.current_position()) < 30.0 {
                    if matches!( shape.get_type(), GameObjectType::Asteroid) {
                        bullet.expire();
                        shape.expire();
                        expl2 = Some( Explosion {
                            time: 0.0,
                            position: shape.current_position(),
                            image: clone_sprite( &self.exp)
                        });
                    }
                }
            }
        }

        if let Some( explosion) = expl2 {
            self.shapes.push( Box::new( explosion));
        }

    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        self.update();
        self.check_collisions();
        
        self.bullets.retain( |x| !x.is_expired());
        self.shapes.retain( |x| !x.is_expired());

        self.ctx.clear_rect(0.0, 0.0, self.game_area.width, self.game_area.height);
        
        self.rocket1.render( &self.ctx);
        self.rocket2.render( &self.ctx);

        for bullet in self.bullets.iter_mut() {
            bullet.render( &self.ctx);
        }
        for shape in self.shapes.iter_mut() {
            shape.render( &self.ctx);
        }
        
        if self.shapes.len() == 0 {
            web_sys::console::log_1(&JsValue::from_str("You win!"));
            self.create_asteroids(20);
        }

        Ok(())
    }

    pub fn up_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("up pressed"));
        self.rocket1.thrust_inc();
    }

    pub fn down_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("down pressed"));
        self.rocket1.thrust_dec();
    }

    pub fn left_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("left pressed"));
        self.rocket1.rotate_left();
    }

    pub fn right_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("right pressed"));
        self.rocket1.rotate_right();
    }

    pub fn space_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("space pressed"));
        let bullet = self.rocket1.fire();

        self.bullets.push( bullet)
    }

    pub fn a_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("a pressed"));
        self.rocket2.rotate_left();
    }

    pub fn d_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("d pressed"));
        self.rocket2.rotate_right();
    }

    pub fn w_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("w pressed"));
        self.rocket2.thrust_inc();
    }

    pub fn x_pressed(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("x pressed"));
        self.rocket2.thrust_dec();
    }

    pub fn s_pressed( &mut self) {
        web_sys::console::log_1(&JsValue::from_str("s pressed"));
        let bullet = self.rocket2.fire();   
        self.shapes.push( bullet);
    }
}
