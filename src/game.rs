use web_sys::CanvasRenderingContext2d;
use crate::vmath::Vector;

pub trait GameObject {
    fn move_t(&mut self, delta_t: f64, game_area: Area);
    fn render(&mut self, ctx: &CanvasRenderingContext2d);

    fn current_position( &mut self) -> Vector;

    fn thrust_inc( &mut self);
    fn thrust_dec( &mut self);
    fn rotate_right( &mut self);
    fn rotate_left( &mut self);
}

#[derive(Clone)]
pub struct Area {
    pub width: f64,
    pub height: f64,
}
pub trait GameArea {
    fn area( &self) -> Area;
}
