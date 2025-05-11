use web_sys::{ CanvasRenderingContext2d, HtmlImageElement, console};
use crate::vmath::Vector;
use std::f64::consts::FRAC_PI_2;
use crate::game::ActiveObject;
use crate::game::GameObject;
use crate::game::Area;
use crate::game::GameObjectType;
use crate::bullet::Bullet;
use std::any::Any;
use crate::game::GameObjectFactory;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Rocket {
    pub score: i32,
    pub damage: i32,
    pub score_pos: Vector,
    pub position: Vector,
    pub rotation: f64,
    pub speed: Vector,
    pub acc: Vector,
    pub thrust: f64,
    pub sprite_on: HtmlImageElement,
    pub sprite_off: HtmlImageElement,
    pub last_shot: i64,
 }

impl Rocket {
    fn update_acc( &mut self) {
        self.acc = Vector::new((self.rotation - FRAC_PI_2).cos(), (self.rotation - FRAC_PI_2).sin()).scale(self.thrust); //.add( &GRAVITY);
    }
}

impl ActiveObject for Rocket {

    fn thrust( &mut self, value : f64) {
        if value >= 0.0 && value <= 1.0 {
            self.thrust = 100.0 * value;
        }

        self.update_acc();
    }


    fn rotate( &mut self, value : f64) {
        self.rotation += value * 0.1;

        self.update_acc();
    }

    fn fire( &mut self, time: i64) -> Option<Box<dyn GameObject>> {
        if self.last_shot == 0 || (time - self.last_shot) > 100 {
            self.last_shot = time;

            let rotvec = Vector::new((self.rotation - FRAC_PI_2).cos(), (self.rotation - FRAC_PI_2).sin()).scale( 25.0);
            let tempo = Vector::new((self.rotation - FRAC_PI_2).cos(), (self.rotation - FRAC_PI_2).sin()).scale( 250.0).add( &self.speed);
            let start = self.position.add( &rotvec);

            let bullet = Bullet {
                expired: false,
                start_position: start,
                position: start,
                speed: tempo,
                rocket: self as *mut Self,
            };

            Some(Box::new(bullet))
        }
        else {
            None
        }
    }   
}

impl GameObject for Rocket {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_type( &self) -> GameObjectType {
        return GameObjectType::Rocket;
    }

    fn current_position(&self) -> Vector {
        self.position
    }

    fn expire( &mut self) {

    }

    fn is_expired( &self) -> bool {
        return false;
    }

    fn move_t(&mut self, delta_t: f64, game_area: Area) {
        self.speed = self.speed.add( &self.acc.scale(delta_t));
        self.position = self.position.add( &self.speed.scale(delta_t));

        if self.position.x > game_area.width {
            self.position.x = 0.0;
        }

        if self.position.x < 0.0 {
            self.position.x = game_area.width;
        }

        if self.position.y > game_area.height {
            self.position.y = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = game_area.height;
        }
    }

    fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        
        let sprite = if self.thrust > 0.0 { &self.sprite_on } else { &self.sprite_off };

        ctx.save();
        ctx.translate(self.position.x, self.position.y).unwrap();          // Move to sprite position
        ctx.rotate( self.rotation).unwrap();        // Rotate around that point
        ctx.draw_image_with_html_image_element_and_dw_and_dh(
            &sprite,
            - (sprite.width() as f64 / 2.0),
            - (sprite.height() as f64 / 2.0),
            sprite.width() as f64,
            sprite.height() as f64,
        ).unwrap();
        ctx.restore();

        // Draw the score at a fixed position
        ctx.set_font("16px sans-serif");
        ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("black"));
        let score_text = format!("Score: {}", self.score);
        ctx.fill_text(&score_text, self.score_pos.x, self.score_pos.y).unwrap();
        let damage_text = format!("Damage: {}", self.damage);
        ctx.fill_text(&damage_text, self.score_pos.x, self.score_pos.y + 20.0).unwrap();
        
    }

    fn radius( &self) -> f64 {
        20.0
    }

    fn collision_with(&mut self, objtype: GameObjectType, _objfactory: &GameObjectFactory) -> Vec<Box<dyn GameObject>> {
        if objtype == GameObjectType::Asteroid {
            self.damage += 100;
        }
        vec![]
    }
}
