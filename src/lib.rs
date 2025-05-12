mod vmath;
mod rocket;
mod asteroid;
mod game;
mod myrand;
mod explosion;
mod bullet;
mod announcer;

use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlImageElement};
use js_sys::Date;
use vmath::Vector;
use rocket::Rocket;
use game::GameObject;
use game::ActiveObject;
use game::Area;
use game::GameObjectFactory;
use crate::announcer::Announcer;

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
    round: i32,
    game_area: Area,
    ctx: CanvasRenderingContext2d,
    time: i64,
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
        let object_factory = GameObjectFactory {
            asteroid_small_image: ass,
            asteroid_medium_image: ams,
            asteroid_large_image: als,
            explosion_image: explosion_sprite,
            rocket_thrust_on_image: rocket_thrust_on,
            rocket_thrust_off_image: rocket_thrust_off
        };
        
       
        let mut objects: Vec<Box<dyn GameObject>> = vec![];

        objects.push( object_factory.create_rocket( Vector { x: game_width / 3.0, y: 200.0 }, Vector { x: 50.0, y: 50.0 }));
        objects.push( object_factory.create_rocket( Vector { x: game_width / 3.0 * 2.0, y: 200.0 }, Vector { x: game_width - 200.0, y: 50.0 }));

        Game {
            round: 1,
            game_area: Area { width: game_width, height: game_height },
            ctx: rendering_context,
            time: Self::now_ms(),
            objfactory: object_factory,
            shapes: objects
        }
    }

    pub fn now_ms() -> i64 {
        Date::now() as i64
    }

    pub fn animate( &mut self)  -> Result<(), JsValue> {
        self.update_game_objects();
        self.check_collisions();
        self.render();

        return Ok(())
    }

    pub fn update_rocket( &mut self, index: usize, thrust: f64, rotate: f64, fire: bool, shield: bool) {
        if index > 1 {
            return;
        }

        if let Some(rocket) = self.shapes[index].as_any_mut().downcast_mut::<Rocket>() {
            let now = Self::now_ms();
            
            rocket.thrust( thrust);
            rocket.rotate( rotate);
            rocket.shield( shield);

            if fire {
                if let Some(bullet) = rocket.fire(now) {
                    self.shapes.push(bullet);
                }
            }
        }
    }

    fn clean_shapes( &mut self) {
        self.shapes.retain( |x| !x.is_expired());

        if self.shapes.len() <= 2 {
            self.start_new_round();
        }
    }

    fn start_new_round( &mut self) {
        let round_text = format!("Round : {}", self.round);
        self.shapes.push(Box::new(Announcer { time: 0.0, position: Vector { x: self.game_area.width / 2.0 - 100.0, y: self.game_area.height / 2.0, }, text: round_text }));
        self.shapes.extend( self.objfactory.create_asteroids(self.round * 2, self.game_area, self.round as f64 * 50.0));
        self.round += 1;
    }

    fn update_game_objects(&mut self) {
        let delta_t = (Self::now_ms() - self.time) as f64 / 1000.0;
        
        self.shapes.iter_mut().for_each(|shape| shape.move_t( delta_t, self.game_area));
        self.time = Self::now_ms();

        self.clean_shapes();
    }

    fn check_collisions(&mut self) {

        let len = self.shapes.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let (left, right) = self.shapes.split_at_mut(j);
                let obj1 = &mut *left[i];  // Dereference the Box
                let obj2 = &mut *right[0]; // Dereference the Box

                if obj1.distance( obj2) < (obj1.radius() + obj2.radius()) {
                    let new_from_obj1 = obj1.collision_with(obj2.get_type(), &self.objfactory);
                    let new_from_obj2 = obj2.collision_with(obj1.get_type(), &self.objfactory);

                    for obj in new_from_obj1.into_iter().chain(new_from_obj2) {
                        self.shapes.push(obj);
                    }
                }
            }
        }
    }

    fn render(&mut self) {
        self.ctx.clear_rect(0.0, 0.0, self.game_area.width, self.game_area.height);
        self.shapes.iter_mut().for_each(|shape| shape.render(&self.ctx));
    }
}
