use web_sys::{HtmlImageElement};
use crate::utils::Vector;
use std::rc::Rc;
use std::cell::RefCell;
use crate::components::asteroid::Asteroid;
use crate::components::asteroid::AsteroidSize;
use crate::Explosion;
use crate::Rocket;
use crate::GameObject;
use crate::Area;
use crate::utils::random_number;
use crate::utils::random_number_max;
use crate::utils::ZERO;

pub struct GameObjectFactory {
    asteroid_small_image: HtmlImageElement,
    asteroid_medium_image: HtmlImageElement,
    asteroid_large_image: HtmlImageElement,
    explosion_image: HtmlImageElement,
    rocket_thrust_on_image: HtmlImageElement,
    rocket_thrust_off_image: HtmlImageElement,
}

impl GameObjectFactory {

    pub fn new(
        asteroid_small_image: HtmlImageElement,
        asteroid_medium_image: HtmlImageElement,
        asteroid_large_image: HtmlImageElement,
        explosion_image: HtmlImageElement,
        rocket_thrust_on_image: HtmlImageElement,
        rocket_thrust_off_image: HtmlImageElement,
    ) -> Self {
        GameObjectFactory {
            asteroid_small_image,
            asteroid_medium_image,
            asteroid_large_image,
            explosion_image,
            rocket_thrust_on_image,
            rocket_thrust_off_image,
        }
    }    

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
            let position = Vector { x: random_number_max( area.width as f64), y: random_number_max( area.height as f64) };
            let speed = Vector { x: max_speed * random_number(), y: max_speed * random_number()};

            asteroids.push( self.create_asteroid_large( position, speed));

            i+=1;
        }

        return asteroids;
    }

    pub fn create_rocket( &self, pos: Vector, score_position: Vector, color: String) -> Rc<RefCell<dyn GameObject>> {
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
            last_shot: 0.0,
            shield_on: false,
            shield_time: 0.0,
            bullet_color: color,
            burst_time: 0.0,
        }))
    }
}
