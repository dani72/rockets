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
use game::GameObjectFactory;

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
    game_area: Area,
    exp : HtmlImageElement,
    ctx: CanvasRenderingContext2d,
    t: i64,
    objfactory: GameObjectFactory,
    shapes: Vec<Box<dyn GameObject>>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(
        game_width: f64,
        game_height: f64,
        ass: HtmlImageElement,
        ams: HtmlImageElement,
        als: HtmlImageElement,
        rocket_thrust_on: HtmlImageElement,
        rocket_thrust_off: HtmlImageElement,
        explosion_sprite: HtmlImageElement,
        rendering_context: CanvasRenderingContext2d,
    ) -> Game {
        let objfactory = GameObjectFactory {
            asteroid_small_image: ass,
            asteroid_medium_image: ams,
            asteroid_large_image: als,
        };
        
        let rocket1 = Rocket {
            name: "Rocket1".to_string(),
            position: Vector { x: 480.0, y: 100.0 },
            rotation: 0.0,
            speed: ZERO,
            acc: ZERO,
            thrust: 0.0,
            sprite_on: clone_sprite(&rocket_thrust_on),
            sprite_off: clone_sprite(&rocket_thrust_off),
            last_shot: 0,
        };
        let rocket2 = Rocket {
            name: "Rocket2".to_string(),
            position: Vector { x: 300.0, y: 50.0 },
            rotation: 0.0,
            speed: ZERO,
            acc: ZERO,
            thrust: 0.0,
            sprite_on: clone_sprite(&rocket_thrust_on),
            sprite_off: clone_sprite(&rocket_thrust_off),
            last_shot: 0,
        };
        
        let mut shapes: Vec<Box<dyn GameObject>> = vec![];

        shapes.push( Box::new( rocket1));
        shapes.push( Box::new( rocket2));

        Game {
            game_area: Area {
                width: game_width,
                height: game_height,
            },
            ctx: rendering_context,
            exp: explosion_sprite,
            t: Self::now_ms(),
            objfactory,
            shapes
        }
    }

    pub fn create_asteroids( &mut self, nof : i32) {
        let mut i = 0;

        while i < nof  {
            let asteroid = self.objfactory.create_asteroid_large( Vector { x: 20.0 * random_number(), y: 15.0 * random_number() }, Vector { x: 5.0 + random_number(), y: 5.0 - random_number() });

            self.shapes.push( asteroid);

            i+=1;
        }
    }

    pub fn now_ms() -> i64 {
        Date::now() as i64
    }

    fn update(&mut self) {
        let delta_t = (Self::now_ms() - self.t) as f64 / 1000.0;
        
        self.shapes.iter_mut().for_each(|shape| shape.move_t( delta_t, self.game_area.clone()));

        self.t = Self::now_ms();
    }

    fn check_collisions(&mut self) {

        let len = self.shapes.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let (left, right) = self.shapes.split_at_mut(j);
                let obj1 = &mut *left[i];  // Dereference the Box
                let obj2 = &mut *right[0]; // Dereference the Box

                if obj1.distance( obj2) < (obj1.radius() + obj2.radius()) {
                    let msg = format!("Collision detected between object {} and {}", i, j);
                    web_sys::console::log_1(&JsValue::from_str(&msg));
                    
                    let new_from_obj1 = obj1.collision_with(obj2.get_type(), &self.objfactory);
                    let new_from_obj2 = obj2.collision_with(obj1.get_type(), &self.objfactory);

                    // Append the resulting objects to self.shapes
                    for obj in new_from_obj1.into_iter().chain(new_from_obj2) {
                        self.shapes.push(obj);
                    }
                }
            }
        }
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        self.update();
        self.check_collisions();
        
        self.shapes.retain( |x| !x.is_expired());

        self.ctx.clear_rect(0.0, 0.0, self.game_area.width, self.game_area.height);
        
        for shape in self.shapes.iter_mut() {
            shape.render( &self.ctx);
        }
        
        if self.shapes.len() <= 2 {
            web_sys::console::log_1(&JsValue::from_str("You win!"));
            self.create_asteroids(20);
        }

        Ok(())
    }

    pub fn update_active_object( &mut self, thrust: f64, rotate: f64, fire: bool) {
        if let Some(rocket) = self.shapes[0].as_any_mut().downcast_mut::<Rocket>() {
            let active: &mut dyn ActiveObject = rocket;
            
            active.thrust( thrust);
            active.rotate( rotate);

            if fire {
                let now = Self::now_ms();
                if let Some(bullet) = active.fire(now) {
                    self.shapes.push(bullet);
                }
            }
        }
    }
}
