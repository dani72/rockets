use web_sys::{ CanvasRenderingContext2d, HtmlImageElement, console};

pub trait GameObject {
    fn move_t(&mut self, delta_t: f64);
    fn render(&mut self, ctx: &CanvasRenderingContext2d);
    fn thrust_on( &mut self);
    fn thrust_off( &mut self);
    fn thrust_right( &mut self);
    fn thrust_left( &mut self);
}
