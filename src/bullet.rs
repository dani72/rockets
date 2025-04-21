use web_sys::{ CanvasRenderingContext2d, HtmlImageElement, console};
use crate::vmath::Vector;
use crate::game::GameObject;
use crate::game::GameObjectType;
use crate::game::Area;
use wasm_bindgen::JsValue;

pub struct Bullet {
    pub start_position: Vector,
    pub position: Vector,
    pub speed: Vector,
 }

impl GameObject for Bullet {

    fn get_type( &self) -> GameObjectType {
        return GameObjectType::Explosion;
    }

    fn current_position(&self) -> Vector {
        return self.position;
    }

    fn is_expired( &self) -> bool {
        return self.position.distance( &self.start_position) > 700.0;
    }

    fn can_collide( &self) -> bool {
        return false;
    }

    fn move_t(&mut self, delta_t: f64, game_area: Area) {
        self.position = self.position.add( &self.speed.scale(delta_t));
    }

    fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.arc( self.position.x, self.position.y, 3.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        ctx.set_fill_style(&JsValue::from_str("red"));
        ctx.fill();
    }

    fn thrust_dec( &mut self) {
    }

    fn thrust_inc( &mut self) {
    }

    fn rotate_right( &mut self) {
    }

    fn rotate_left( &mut self) {
    }
}
