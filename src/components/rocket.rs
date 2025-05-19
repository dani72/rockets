use web_sys::{ CanvasRenderingContext2d, HtmlImageElement};
use crate::utils::Vector;
use std::f64::consts::FRAC_PI_2;
use crate::engine::{GameObject, GameObjectType, GameObjectFactory, Area, GamepadState};
use crate::components::bullet::Bullet;
use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;

const MAX_SHIELD_TIME: f64 = 2.0;
const MAX_SHIELD_STROKE_WIDTH: f64 = 6.0;
const MAX_BURST_TIME: f64 = 2.5;

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
    pub last_shot: f64,
    pub shield_on: bool,
    pub shield_time: f64,
    pub bullet_color: String,
    pub burst_time : f64
 }

impl Rocket {
    pub fn update( &mut self, delta_t: f64, state: &GamepadState) -> Vec<Rc<RefCell<dyn GameObject>>> {
        self.thrust( state.thrust);
        self.rotate( state.rotate);

        if state.shield {
            self.shield_on( delta_t);
        }
        else {
            self.shield_off( delta_t);
        }

        if state.fire {
            return self.fire_on( delta_t)
        }
        else {
            self.fire_off( delta_t)
        }

        vec![]
    }
        
    pub fn fire_off( &mut self, delta_t: f64) {
        self.last_shot = 0.0;

        if self.burst_time > 0.0 {
            self.burst_time -= delta_t;
        }
    }

    pub fn fire_on( &mut self, delta_t: f64) -> Vec<Rc<RefCell<dyn GameObject>>> {
        if self.burst_time < MAX_BURST_TIME {
            self.burst_time += delta_t;
        }

        if (self.last_shot == 0.0 || self.last_shot > 0.2) && (self.burst_time < MAX_BURST_TIME) {
            self.last_shot = 0.01;

            let rotvec = Vector::new((self.rotation - FRAC_PI_2).cos(), (self.rotation - FRAC_PI_2).sin()).scale( 25.0);
            let tempo = Vector::new((self.rotation - FRAC_PI_2).cos(), (self.rotation - FRAC_PI_2).sin()).scale( 250.0).add( &self.speed);
            let start = self.position.add( &rotvec);

            let bullet = Bullet {
                expired: false,
                start_position: start,
                position: start,
                speed: tempo,
                color: self.bullet_color.to_string(),
                rocket: self as *mut Self,
            };

            return vec![ Rc::new( RefCell::new( bullet))];
        }
        else {
            self.last_shot += delta_t;
        }

        vec![]

    }   

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

    fn shield_on( &mut self, delta_t: f64) {
        self.shield_on = true;

        if self.shield_time < MAX_SHIELD_TIME {
            self.shield_time += delta_t;
        }
    }

    fn shield_off( &mut self, delta_t: f64) {
        self.shield_on = false;

        if self.shield_time > 0.0 {
            self.shield_time -= delta_t;
        }
    }

    fn update_acc( &mut self) {
        self.acc = Vector::new((self.rotation - FRAC_PI_2).cos(), (self.rotation - FRAC_PI_2).sin()).scale(self.thrust); //.add( &GRAVITY);
    }

    fn is_shield_active( &self) -> bool {
        return self.shield_on && (self.shield_time > 0.0) && (self.shield_time < MAX_SHIELD_TIME);
    }

    fn render_score( &self, ctx: &CanvasRenderingContext2d) {
        ctx.set_font("16px sans-serif");
        ctx.set_fill_style_str( "black");
        let score_text = format!("Score: {}", self.score);
        ctx.fill_text(&score_text, self.score_pos.x, self.score_pos.y).unwrap();
        let damage_text = format!("Damage: {}", self.damage);
        ctx.fill_text(&damage_text, self.score_pos.x, self.score_pos.y + 20.0).unwrap();

        // Draw burst time remaining bar under score
        let max_width = 100.0;
        let bar_width = ((MAX_BURST_TIME - self.burst_time) / MAX_BURST_TIME) * max_width;
        let bar_height = 5.0;
        let bar_x = self.score_pos.x;
        let bar_y = self.score_pos.y + 40.0;

        ctx.set_fill_style_str( "red");
        ctx.fill_rect(bar_x, bar_y, bar_width, bar_height);
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

        if self.shield_on {
            if self.shield_time < MAX_SHIELD_TIME {
                self.shield_time += delta_t;
            }
        }
        else {
            if self.shield_time > 0.0 {
                self.shield_time -= delta_t;
            }
        }
    }

    fn render(&self, ctx: &CanvasRenderingContext2d) {
        
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

        // Draw a shield circle
        if self.is_shield_active() && self.shield_time > 0.0 {
            ctx.begin_path();
            ctx.arc(0.0, 0.0, self.radius() + 10.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
            ctx.set_stroke_style_str( "rgba(0, 200, 255, 0.5)");
            ctx.set_line_width((MAX_SHIELD_TIME - (self.shield_time)) * (MAX_SHIELD_STROKE_WIDTH / MAX_SHIELD_TIME));
            ctx.stroke();
        }

        ctx.restore();

        self.render_score(ctx);
    }

    fn radius( &self) -> f64 {
        20.0
    }

    fn collision_with(&mut self, objtype: GameObjectType, objfactory: &GameObjectFactory) -> Vec<Rc<RefCell<dyn GameObject>>> {
        if objtype == GameObjectType::Asteroid {
            if !self.is_shield_active() {
                self.damage += 100;
            }

            return vec![objfactory.create_explosion(self.position)];
        }
        else if objtype == GameObjectType::Bullet {
            if !self.is_shield_active() {
                self.damage += 50;
            }
            else {
                self.shield_time += 0.01;
            }
        }
        else if objtype == GameObjectType::Rocket {
            if !self.is_shield_active() {
                self.damage += 500;
            }
            else {
                self.shield_time += 0.05;
            }
        }
        vec![]
    }
}
