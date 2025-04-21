use web_sys::{ CanvasRenderingContext2d, HtmlImageElement, console};
use crate::vmath::Vector;
use crate::game::GameObject;
use crate::game::Area;

pub struct Asteroid {
    pub name: String,
    pub position: Vector,
    pub rotation: f64,
    pub speed: Vector,
    pub acc: Vector,
    pub image: HtmlImageElement,
 }

impl Asteroid {
    fn status( &mut self) {
        console::log_1( &format!("{}: x = {}, y = {} (Speed {}, {}) (Acc {} {})", self.name, self.position.x, self.position.y, self.speed.x, self.speed.y, self.acc.x, self.acc.y).into());
    }
}

impl GameObject for Asteroid {
    fn move_t(&mut self, delta_t: f64, game_area: Area) {
        self.speed = self.speed.add( &self.acc.scale(delta_t));
        self.position = self.position.add( &self.speed.scale(delta_t));

        if self.position.x > game_area.width {
            self.position.x = 0.0;
        }

        if self.position.x < 0.0 {
            self.position.x = 1000.0;
        }

        if self.position.y > game_area.height {
            self.position.y = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = 600.0;
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

    fn thrust_dec( &mut self) {
    }

    fn thrust_inc( &mut self) {
    }

    fn rotate_right( &mut self) {
    }

    fn rotate_left( &mut self) {
    }
}
