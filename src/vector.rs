use crate::body::Body;
use num_traits::{Float, Pow};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T>
where
    T: Copy
        + Clone
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Float
        + Pow<T, Output = T>,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn from_body(a: &Body<T>, b: &Body<T>) -> Vector<T> {
        let relative_x = a.x - b.x;
        let relative_y = a.y - b.y;
        Vector::new(relative_x, relative_y)
    }

    pub fn magnitude(&self, other: Vector<T>) -> T {
        let mx = self.x - other.x;
        let my = self.y - other.y;
        let m = ((mx).powi(2) + (my).powi(2)).sqrt();
        m
    }
}
