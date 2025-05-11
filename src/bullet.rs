use web_sys::CanvasRenderingContext2d;
use crate::vmath::Vector;
use crate::game::GameObject;
use crate::game::GameObjectType;
use crate::game::Area;
use std::any::Any;
use crate::game::GameObjectFactory;
use wasm_bindgen::JsValue;

pub struct Bullet {
    pub expired: bool,
    pub start_position: Vector,
    pub position: Vector,
    pub speed: Vector,
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

    fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.arc( self.position.x, self.position.y, 3.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        ctx.set_fill_style_str( "red");
    
        ctx.fill();
    }

    fn radius( &self) -> f64 {
        return 3.0;
    }

    fn collision_with(&mut self, objtype: GameObjectType, objfactory: &GameObjectFactory) -> Vec<Box<dyn GameObject>> {

        if objtype == GameObjectType::Asteroid {
            self.expire();
            return vec![objfactory.create_explosion(self.position.clone())];
        }

        vec![]
    }
}
