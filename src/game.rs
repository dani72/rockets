use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use crate::vmath::Vector;
use std::any::Any;
use crate::asteroid::Asteroid;

pub enum GameObjectType {
    Asteroid,
    Rocket,
    Explosion,
    Bullet,
}

pub trait GameObject : Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_type( &self) -> GameObjectType;

    fn move_t(&mut self, delta_t: f64, game_area: Area);
    fn render(&mut self, ctx: &CanvasRenderingContext2d);
    fn collision_with(&mut self, objtype: GameObjectType, objfactory: &GameObjectFactory) -> Vec<Box<dyn GameObject>>;

    fn current_position( &self) -> Vector;
    fn radius( &self) -> f64;

    fn is_expired( &self) -> bool;
    fn expire( &mut self);

    fn distance( &self, other: &dyn GameObject) -> f64 {
        self.current_position().distance( &other.current_position())
    }
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

pub struct GameObjectFactory {
    pub asteroid_small_image: HtmlImageElement,
    pub asteroid_medium_image: HtmlImageElement,
    pub asteroid_large_image: HtmlImageElement,
}

impl GameObjectFactory {
    pub fn create_asteroid_small( &mut self, position: Vector) -> Box<dyn GameObject> {
        Box::new(Asteroid {
            expired: false,
            position: position,
            rotation: 0.0,
            speed: Vector::new(0.0, 0.0),
            acc: Vector::new(0.0, 0.0),
            radius: 10.0,
            image: self.asteroid_small_image.clone(),
        })
    }

    pub fn create_asteroid_medium( &mut self, position: Vector) -> Box<dyn GameObject> {
        Box::new(Asteroid {
            expired: false,
            position: position,
            rotation: 0.0,
            speed: Vector::new(0.0, 0.0),
            acc: Vector::new(0.0, 0.0),
            radius: 20.0,
            image: self.asteroid_medium_image.clone(),
        })
    }

    pub fn create_asteroid_large( &mut self, position: Vector, speed: Vector) -> Box<dyn GameObject> {
        Box::new(Asteroid {
            expired: false,
            position: position,
            rotation: 0.0,
            speed: speed,
            acc: Vector::new(0.0, 0.0),
            radius: 30.0,
            image: self.asteroid_large_image.clone(),
        })
    }
}
