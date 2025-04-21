use web_sys::{ CanvasRenderingContext2d, HtmlImageElement, console};
use crate::vmath::Vector;
use crate::game::GameObject;
use crate::game::Area;

pub struct Explosion {
    pub time: f64,
    pub position: Vector,
    pub image: HtmlImageElement,
 }

impl GameObject for Explosion {

    fn current_position(&self) -> Vector {
        self.position
    }

    fn move_t(&mut self, delta_t: f64, game_area: Area) {
        self.time += delta_t;
    }

    fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        if self.time < 2.0 {
            ctx.save();
            ctx.translate(self.position.x, self.position.y).unwrap();          // Move to sprite position
//            ctx.rotate( self.rotation).unwrap();        // Rotate around that point
            ctx.draw_image_with_html_image_element_and_dw_and_dh(
                &self.image,
                - (self.image.width() as f64 / 2.0),
                - (self.image.height() as f64 / 2.0),
                self.image.width() as f64,
                self.image.height() as f64,
            ).unwrap();
            ctx.restore();
        }
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
