use web_sys::CanvasRenderingContext2d;
use crate::vmath::Vector;

pub enum GameObjectType {
    Asteroid,
    Rocket,
    Explosion,
    Bullet,
}

pub trait GameObject {
    fn move_t(&mut self, delta_t: f64, game_area: Area);
    fn render(&mut self, ctx: &CanvasRenderingContext2d);

    fn get_type( &self) -> GameObjectType;
    fn current_position( &self) -> Vector;
    fn is_expired( &self) -> bool;

    fn expire( &mut self);
}

pub trait ActiveObject : GameObject {
    fn rotate( &mut self, value: f64);
    fn thrust( &mut self, value: f64);
    fn fire( &mut self, time: i64) -> Option<Box<dyn GameObject>>;    
}

#[derive(Clone)]
pub struct Area {
    pub width: f64,
    pub height: f64,
}
