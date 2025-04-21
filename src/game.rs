use web_sys::CanvasRenderingContext2d;

pub trait GameObject {
    fn move_t(&mut self, delta_t: f64);
    fn render(&mut self, ctx: &CanvasRenderingContext2d);
    fn thrust_inc( &mut self);
    fn thrust_dec( &mut self);
    fn rotate_right( &mut self);
    fn rotate_left( &mut self);
}

pub trait GameArea {
    fn width(&self) -> f64;
    fn height(&self) -> f64;
}
