use web_sys::{ CanvasRenderingContext2d, HtmlImageElement, console};
use crate::vmath::Vector;
use std::f64::consts::FRAC_PI_2;
use crate::game::ActiveObject;
use crate::game::GameObject;
use crate::game::Area;
use crate::game::GameObjectType;
use crate::bullet::Bullet;

pub struct Rocket {
    pub name: String,
    pub position: Vector,
    pub rotation: f64,
    pub speed: Vector,
    pub acc: Vector,
    pub thrust: f64,
    pub sprite_on: HtmlImageElement,
    pub sprite_off: HtmlImageElement
 }

impl Rocket {
    fn status( &mut self) {
        console::log_1( &format!("{}: x = {}, y = {} (Speed {}, {}) (Acc {} {}) (Thrust {})", self.name, self.position.x, self.position.y, self.speed.x, self.speed.y, self.acc.x, self.acc.y, self.thrust).into());
    }

    fn update_acc( &mut self) {
        self.acc = Vector::new((self.rotation - FRAC_PI_2).cos(), (self.rotation - FRAC_PI_2).sin()).scale(self.thrust); //.add( &GRAVITY);
    }
}

impl ActiveObject for Rocket {

    fn thrust_dec( &mut self) {
        if self.thrust > 0.0 {
            self.thrust -= 1.0;
        }

        self.update_acc();
        self.status();
    }

    fn thrust_inc( &mut self) {
        if self.thrust < 20.0 {
            self.thrust += 1.0;
        }

        self.update_acc();
        self.status();
    }

    fn rotate_right( &mut self) {
        self.rotation += 0.1;

        self.update_acc();
        self.status();
    }

    fn rotate_left( &mut self) {
        self.rotation -= 0.1;

        self.update_acc();
        self.status();
    }

    fn fire( &mut self) -> Box<dyn GameObject> {
        let bullet = Bullet {
            expired: false,
            start_position: self.position.clone(),
            position: self.position.clone(),
            speed: Vector::new((self.rotation - FRAC_PI_2).cos(), (self.rotation - FRAC_PI_2).sin()).scale( 250.0).add( &self.speed)
        };

        Box::new( bullet)
    }   
}

impl GameObject for Rocket {

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
    }
}
