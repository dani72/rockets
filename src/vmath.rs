#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64
}

pub const GRAVITY: Vector = Vector{ x: 0.0, y: 9.81};

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::zero()
        } else {
            Self::new(self.x / len, self.y / len)
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn scale(&self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }        
}
