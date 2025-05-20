use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use js_sys::Date;
use crate::utils::Vector;
use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use crate::components::GameObjectFactory;
use crate::components::Countdown;
use crate::components::Rocket;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct GamepadState {
    pub gamepad_index: usize,
    pub rocket_index: usize,
    pub rotate: f64,
    pub thrust: f64,
    pub shield: bool,
    pub fire: bool,
}

#[wasm_bindgen]
impl GamepadState {
   #[wasm_bindgen(constructor)]
    pub fn new() -> GamepadState {
        GamepadState {
            gamepad_index: 0,
            rocket_index: 0,
            rotate: 0.0,
            thrust: 0.0,
            shield: false,
            fire: false,
        }
    }
}

#[wasm_bindgen]
pub struct GamepadStates {
    inner: Vec<GamepadState>,
}

#[wasm_bindgen]
impl GamepadStates {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GamepadStates {
        GamepadStates {
            inner: Vec::new(),
        }
    }

    pub fn push(&mut self, state: &GamepadState) {
        self.inner.push( state.clone());
    }

    pub fn get(&self, index: usize) -> Option<GamepadState> {
        self.inner.get(index).cloned()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[derive(PartialEq, Eq)]
pub enum GameObjectType {
    Asteroid,
    Rocket,
    Explosion,
    Bullet,
    Announcer,
    Countdown,
}

pub trait GameObject : Any {
    fn as_any( &self) -> &dyn Any;
    fn as_any_mut( &mut self) -> &mut dyn Any;

    fn get_type( &self) -> GameObjectType;
    fn current_position( &self) -> Vector;
    fn radius( &self) -> f64;

    fn is_expired( &self) -> bool;
    fn expire( &mut self);

    fn move_t( &mut self, delta_t: f64, game_area: Area);
    fn collision_with( &mut self, objtype: GameObjectType, objfactory: &GameObjectFactory) -> Vec<Rc<RefCell<dyn GameObject>>>;
    fn render( &self, ctx: &CanvasRenderingContext2d);

    fn distance( &self, other: &dyn GameObject) -> f64 {
        self.current_position().distance( &other.current_position())
    }
}

#[derive(Clone, Copy)]
pub struct Area {
    pub width: f64,
    pub height: f64,
}


#[wasm_bindgen]
pub struct Game {
    round: i32,
    game_area: Area,
    ctx: CanvasRenderingContext2d,
    time: i64,
    objfactory: Rc<RefCell<GameObjectFactory>>,
    shapes: Vec<Rc<RefCell<dyn GameObject>>>,
    number_of_rockets: usize
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(
        game_width: f64,
        game_height: f64,
        ass: HtmlImageElement,
        ams: HtmlImageElement,
        als: HtmlImageElement,
        rocket_thrust_on: HtmlImageElement,
        rocket_thrust_off: HtmlImageElement,
        explosion_sprite: HtmlImageElement,
        rendering_context: CanvasRenderingContext2d,
    ) -> Game {
        let object_factory = Rc::new(RefCell::new( GameObjectFactory::new(
            ass,
            ams,
            als,
            explosion_sprite,
            rocket_thrust_on,
            rocket_thrust_off,
        )));
        
        Game {
            round: 1,
            game_area: Area { width: game_width, height: game_height },
            ctx: rendering_context,
            time: Self::now_ms(),
            objfactory: object_factory,
            shapes: vec![],
            number_of_rockets: 0
        }
    }

    pub fn now_ms() -> i64 {
        Date::now() as i64
    }

    pub fn animate_frame( &mut self, states: &GamepadStates)  -> Result<(), JsValue> {
        let now = Self::now_ms();
        let delta_t = (now - self.time) as f64 / 1000.0;

        self.update_rockets( delta_t, states);
        self.update_game_objects( delta_t);
        self.check_collisions();
        self.render();

        self.time = now;

        return Ok(())
    }

    pub fn create_rocket( &mut self, color: String) -> usize {
        let position = Vector { x: (self.game_area.width / 3.0) + self.number_of_rockets as f64 * 50.0, y: 200.0 };
        let score_position = Vector { x: 50.0 + self.number_of_rockets as f64 * 150.0, y: 50.0 };
        let rocket = self.objfactory.borrow().create_rocket( position, score_position, color);

        self.shapes.insert( self.number_of_rockets, rocket);
        self.number_of_rockets += 1;

        return self.number_of_rockets - 1;
    }

    fn update_rockets( &mut self, delta_t: f64, states: &GamepadStates) {
        for i in 0..states.len() {
            if let Some(state) = states.get(i) {
                self.update_rocket( delta_t, &state);
            }
        }
    }

    fn update_rocket( &mut self, delta_t: f64, state: &GamepadState) {
        let mut bullets: Vec<Rc<RefCell<dyn GameObject>>> = vec![];

        if let Some( rocket) = self.shapes[state.rocket_index].borrow_mut().as_any_mut().downcast_mut::<Rocket>() {
            bullets.extend( rocket.update( delta_t, state));
        }

        self.shapes.extend( bullets);
    }

    fn clean_shapes( &mut self) {
        self.shapes.retain( |x| !x.borrow().is_expired());

        let nof_asteroids = self.shapes.iter().filter(|obj| obj.borrow().get_type() == GameObjectType::Asteroid).count();
        let nof_countdowns = self.shapes.iter().filter(|obj| obj.borrow().get_type() == GameObjectType::Countdown).count();
        if nof_asteroids == 0 && nof_countdowns == 0{
            self.start_new_round();
        }
    }

    fn start_new_round( &mut self) {
        let countdown = Rc::new( RefCell::new( Countdown { game: self as *mut Self, time: 0.0, position: Vector { x: self.game_area.width / 2.0 - 20.0, y: self.game_area.height / 2.0 - 8.0 }, count: 5, text: "5".to_string() }));

        self.shapes.push( countdown);
        self.round += 1;
    }

    pub fn spawn_asteroids( &mut self) {
        self.shapes.extend( self.objfactory.borrow().create_asteroids( self.round * 2, self.game_area, self.round as f64 * 50.0));
    }

    fn update_game_objects( &mut self, delta_t : f64) {
        self.shapes.iter_mut().for_each(|shape| shape.borrow_mut().move_t( delta_t, self.game_area));
        self.clean_shapes();
    }

    fn check_collisions( &mut self) {
        let mut objects : Vec<Rc<RefCell<dyn GameObject>>> = vec![];

        let len = self.shapes.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let (left, right) = self.shapes.split_at_mut(j);
                let obj1 = &left[i];
                let obj2 = &right[0];

                if obj1.borrow().distance( &*obj2.borrow()) < (obj1.borrow().radius() + obj2.borrow().radius()) {
                    objects.extend( obj1.borrow_mut().collision_with( obj2.borrow().get_type(), &self.objfactory.borrow()));
                    objects.extend( obj2.borrow_mut().collision_with( obj1.borrow().get_type(), &self.objfactory.borrow()));
                }
            }
        }

        self.shapes.extend(objects);
    }

    fn render( &self) {
        self.ctx.clear_rect(0.0, 0.0, self.game_area.width, self.game_area.height);
        self.shapes.iter().for_each(|shape| shape.borrow().render(&self.ctx));
    }
}
