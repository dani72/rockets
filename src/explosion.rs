use web_sys::{ CanvasRenderingContext2d, HtmlImageElement};
use crate::vmath::Vector;
use crate::game::GameObject;
use crate::game::GameObjectType;
use crate::game::Area;

pub struct Explosion {
    pub time: f64,
    pub position: Vector,
    pub image: HtmlImageElement,
 }

impl GameObject for Explosion {

    fn get_type( &self) -> GameObjectType {
        return GameObjectType::Explosion;
    }

    fn current_position(&self) -> Vector {
        self.position
    }

    fn is_expired( &self) -> bool {
        return self.time > 1.0;
    }

    fn expire( &mut self) {

    }

    fn move_t(&mut self, delta_t: f64, _game_area: Area) {
        self.time += delta_t;
    }

    fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        ctx.save();
        ctx.translate(self.position.x, self.position.y).unwrap();          // Move to sprite position
        let _ = ctx.scale( self.time / 1.0, self.time / 1.0);
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
