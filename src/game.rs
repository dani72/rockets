use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use crate::vmath::Vector;
use crate::myrand::random_number_max;
use crate::myrand::random_number;
use std::any::Any;
use crate::asteroid::Asteroid;
use crate::asteroid::AsteroidSize;
use crate::explosion::Explosion;
use crate::vmath::ZERO;
use std::rc::Rc;
use std::cell::RefCell;
use crate::rocket::Rocket;

#[derive(PartialEq, Eq)]
pub enum GameObjectType {
    Asteroid,
    Rocket,
    Explosion,
    Bullet,
    Announcer
}

pub trait GameObject : Any {
    fn as_any( &self) -> &dyn Any;
    fn as_any_mut( &mut self) -> &mut dyn Any;

    fn get_type( &self) -> GameObjectType;
    fn current_position( &self) -> Vector;
    fn radius( &self) -> f64;

    fn is_expired( &self) -> bool;
    fn expire( &mut self);

    fn move_t( &mut self, delta_t: f64, game_area: Area);
    fn collision_with( &mut self, objtype: GameObjectType, objfactory: &GameObjectFactory) -> Vec<Rc<RefCell<dyn GameObject>>>;
    fn render( &self, ctx: &CanvasRenderingContext2d);

    fn distance( &self, other: &dyn GameObject) -> f64 {
        self.current_position().distance( &other.current_position())
    }
}

#[derive(Clone, Copy)]
pub struct Area {
    pub width: f64,
    pub height: f64,
}

pub struct GameObjectFactory {
    pub asteroid_small_image: HtmlImageElement,
    pub asteroid_medium_image: HtmlImageElement,
    pub asteroid_large_image: HtmlImageElement,
    pub explosion_image: HtmlImageElement,
    pub rocket_thrust_on_image: HtmlImageElement,
    pub rocket_thrust_off_image: HtmlImageElement,
}

impl GameObjectFactory {
    pub fn create_asteroid_small( &self, position: Vector, speed: Vector) -> Rc<RefCell<dyn GameObject>> {
        Rc::new( RefCell::new( Asteroid {
            size: AsteroidSize::Small,
            expired: false,
            position: position,
            rotation: 0.0,
            speed: speed,
            acc: Vector::new(0.0, 0.0),
            radius: 10.0,
            image: self.asteroid_small_image.clone(),
        }))
    }

    pub fn create_asteroid_medium( &self, position: Vector, speed: Vector) -> Rc<RefCell<dyn GameObject>> {
        Rc::new( RefCell::new( Asteroid {
            size: AsteroidSize::Medium,
            expired: false,
            position: position,
            rotation: 0.0,
            speed: speed,
            acc: Vector::new(0.0, 0.0),
            radius: 20.0,
            image: self.asteroid_medium_image.clone(),
        }))
    }

    pub fn create_asteroid_large( &self, position: Vector, speed: Vector) -> Rc<RefCell<dyn GameObject>> {
        Rc::new( RefCell::new( Asteroid {
            size: AsteroidSize::Large,
            expired: false,
            position: position,
            rotation: 0.0,
            speed: speed,
            acc: Vector::new(0.0, 0.0),
            radius: 30.0,
            image: self.asteroid_large_image.clone(),
        }))
    }

    pub fn create_explosion( &self, position: Vector) -> Rc<RefCell<dyn GameObject>> {
        Rc::new( RefCell::new( Explosion {
            time: 0.0f64,
            position: position,
            image: self.explosion_image.clone(),
        }))
    }

    pub fn create_asteroids( &self, nof : i32, area: Area, max_speed: f64) -> Vec<Rc<RefCell<dyn GameObject>>> {
        let mut i = 0;
        let mut asteroids = vec![];

        while i < nof  {
            let asteroid = self.create_asteroid_large( Vector { x: random_number_max( area.width as f64), y: random_number_max( area.height as f64) }, Vector { x: max_speed * random_number(), y: max_speed * random_number()});
            asteroids.push(asteroid);

            i+=1;
        }

        return asteroids;
    }

    pub fn create_rocket( &self, pos: Vector, score_position: Vector) -> Rc<RefCell<dyn GameObject>> {
        Rc::new( RefCell::new( Rocket {
            score: 0,
            damage: 0,
            score_pos: score_position,
            position: pos,
            rotation: 0.0,
            speed: ZERO,
            acc: ZERO,
            thrust: 0.0,
            sprite_on: self.rocket_thrust_on_image.clone(),
            sprite_off: self.rocket_thrust_off_image.clone(),
            last_shot: 0,
            shield_on: false,
            shield_time: 0.0,
        }))
    }
}
