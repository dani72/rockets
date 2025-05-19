use web_sys::{ CanvasRenderingContext2d, HtmlImageElement};
use crate::utils::Vector;
use crate::engine::{GameObject, GameObjectType, GameObjectFactory, Area};
use crate::utils::random_number;
use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq, Eq)]
pub enum AsteroidSize {
    Small,
    Medium,
    Large,
}

pub struct Asteroid {
    pub size: AsteroidSize,
    pub expired: bool,
    pub position: Vector,
    pub rotation: f64,
    pub speed: Vector,
    pub acc: Vector,
    pub radius: f64,
    pub image: HtmlImageElement,
 }

impl Asteroid {
}

impl GameObject for Asteroid {
    fn as_any( &self) -> &dyn Any {
        self
    }

    fn as_any_mut( &mut self) -> &mut dyn Any {
        self
    }

    fn get_type( &self) -> GameObjectType {
        return GameObjectType::Asteroid;
    }

    fn current_position( &self) -> Vector {
        self.position
    }

    fn expire( &mut self) {
        self.expired = true;
    }

    fn is_expired( &self) -> bool {
        return self.expired;
    }

    fn move_t( &mut self, delta_t: f64, game_area: Area) {
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

    fn render( &self, ctx: &CanvasRenderingContext2d) {
        ctx.save();
        ctx.translate(self.position.x, self.position.y).unwrap();          // Move to sprite position
        ctx.rotate( self.rotation).unwrap();        // Rotate around that point
        ctx.draw_image_with_html_image_element_and_dw_and_dh(
            &self.image,
            - (self.image.width() as f64 / 2.0),
            - (self.image.height() as f64 / 2.0),
            self.image.width() as f64,
            self.image.height() as f64,
        ).unwrap();
        ctx.restore();
    }

    fn radius( &self) -> f64 {
        return self.radius;
    }

    fn collision_with( &mut self, objtype: GameObjectType, objfactory: &GameObjectFactory) -> Vec<Rc<RefCell<dyn GameObject>>> {
        if objtype == GameObjectType::Bullet || objtype == GameObjectType::Rocket {
            let mut result = Vec::new();

            if self.size == AsteroidSize::Large {
                let base_dir = self.speed.normalize();
                let perp = Vector { x: -base_dir.y, y: base_dir.x };
                for _ in 0..2 {
                    let angle = (random_number() - 0.5) * std::f64::consts::PI / 2.0;
                    let impulse = perp.rotate(angle).scale((random_number() * 50.0 + 30.0) * 0.5); // mass scaling: quarter mass => half speed
                    let new_speed = self.speed.add(&impulse);
                    result.push(objfactory.create_asteroid_medium(self.position, new_speed));
                }
            } 
            else if self.size == AsteroidSize::Medium {
                let base_dir = self.speed.normalize();
                let perp = Vector { x: -base_dir.y, y: base_dir.x };
                for _ in 0..2 {
                    let angle = (random_number() - 0.5) * std::f64::consts::PI / 2.0;
                    let impulse = perp.rotate(angle).scale(random_number() * 50.0 + 30.0); // slightly faster for smaller fragments
                    let new_speed = self.speed.add(&impulse);
                    result.push(objfactory.create_asteroid_small(self.position, new_speed));
                }
            }

            self.expire();

            return result;
        }
        vec![]
    }
}
