use web_sys::CanvasRenderingContext2d;
use crate::utils::Vector;
use crate::engine::{GameObject, GameObjectType, Area};
use crate::components::Rocket;
use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use crate::GameObjectFactory;

pub struct Bullet {
    pub expired: bool,
    pub start_position: Vector,
    pub position: Vector,
    pub speed: Vector,
    pub color: String,
    pub rocket: *mut Rocket,
 }

impl GameObject for Bullet {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }


    fn get_type( &self) -> GameObjectType {
        return GameObjectType::Bullet;
    }

    fn current_position(&self) -> Vector {
        return self.position;
    }

    fn expire( &mut self) {
        self.expired = true;
    }

    fn is_expired( &self) -> bool {
        return self.expired || (self.position.distance( &self.start_position) > 700.0);
    }

    fn move_t(&mut self, delta_t: f64, _game_area: Area) {
        self.position = self.position.add( &self.speed.scale(delta_t));
    }

    fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.arc( self.position.x, self.position.y, 3.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        ctx.set_fill_style_str( &self.color);
    
        ctx.fill();
    }

    fn radius( &self) -> f64 {
        return 3.0;
    }

    fn collision_with(&mut self, objtype: GameObjectType, objfactory: &GameObjectFactory) -> Vec<Rc<RefCell<dyn GameObject>>> {

        if objtype == GameObjectType::Asteroid  || objtype == GameObjectType::Rocket {
            self.expire();

            unsafe {
                (*self.rocket).score += 100;
            }

            return vec![objfactory.create_explosion(self.position)];
        }

        vec![]
    }
}
