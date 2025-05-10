use web_sys::{ CanvasRenderingContext2d, HtmlImageElement};
use crate::game::GameObjectType;
use crate::vmath::Vector;
use crate::game::GameObject;
use crate::game::Area;
use crate::game::GameObjectFactory;
use std::any::Any;

pub struct Asteroid {
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
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_type( &self) -> GameObjectType {
        return GameObjectType::Asteroid;
    }

    fn current_position(&self) -> Vector {
        self.position
    }

    fn expire( &mut self) {
        self.expired = true;
    }

    fn is_expired( &self) -> bool {
        return self.expired;
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


    fn collision_with(&mut self, _objtype: GameObjectType, objfactory: &GameObjectFactory) -> Vec<Box<dyn GameObject>> {
        vec![]
    }
}
