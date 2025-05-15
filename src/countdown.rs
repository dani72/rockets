use web_sys::{ CanvasRenderingContext2d, HtmlImageElement};
use crate::vmath::Vector;
use crate::game::GameObject;
use crate::game::GameObjectType;
use crate::game::Area;
use std::any::Any;
use crate::game::GameObjectFactory;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Countdown {
    pub time: f64,
    pub position: Vector,
    pub count : i32,
    pub text: String,
 }

impl Countdown {
    fn format_count( &mut self) {
        self.text = self.count.to_string();
    }
}

impl GameObject for Countdown {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_type( &self) -> GameObjectType {
        return GameObjectType::Announcer;
    }

    fn current_position(&self) -> Vector {
        self.position
    }

    fn is_expired( &self) -> bool {
        return self.count == -1
    }

    fn expire( &mut self) {

    }

    fn move_t(&mut self, delta_t: f64, _game_area: Area) {
        self.time += delta_t;

        if self.time > 1.0 {
            self.count -= 1;
            self.time = 0.0;
            self.format_count();
        }
    }

    fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_font("48px sans-serif");
        ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("black"));
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");
        ctx.set_global_alpha( 1.0 - self.time);
        ctx.fill_text(&self.text, self.position.x, self.position.y).unwrap();
        ctx.set_global_alpha(1.0);
    }

    fn radius( &self) -> f64 {
        return 10.0;
    }

    fn collision_with(&mut self, _objtype: GameObjectType, _objfactory: &GameObjectFactory) -> Vec<Rc<RefCell<dyn GameObject>>> {
        vec![]
    }
}
