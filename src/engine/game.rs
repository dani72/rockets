use web_sys::{CanvasRenderingContext2d};
use crate::utils::Vector;
use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use crate::components::GameObjectFactory;

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

