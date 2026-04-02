use num_traits::{Float, Pow};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Body<T> {
    pub x: T,
    pub y: T,
}

impl<T> Body<T>
where
    T: Copy
        + Clone
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Float
        + Pow<T, Output = T>,
{
    /// Create a new Body(Point)
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Get the X of a Body
    pub fn x(&self) -> T {
        self.x
    }

    /// Get the Y of a Body
    pub fn y(&self) -> T {
        self.y
    }

    /// Calculates de distance between 2 points. this makes the Point A be at 0,0
    pub fn distance_to(&self, other: &Body<T>) -> T {
        let distance_x = self.x - other.x;
        let distance_y = self.y - other.y;
        ((distance_x).powi(2) + (distance_y).powi(2)).sqrt()
    }

    /// The relative distance/position
    pub fn relative_to(&self, other: &Body<T>) -> (T, T) {
        let relative_x = self.x - other.x;
        let relative_y = self.x - other.x;
        (relative_x, relative_y)
    }

    /// Calculates de distance WITHOUT the sqrt() function
    pub fn distance_squared_to(&self, other: &Body<T>) -> T {
        let distance_x = self.x - other.x;
        let distance_y = self.y - other.y;
        (distance_x).powi(2) + (distance_y).powi(2)
    }

    /// Calculates the absolute distance between 2 Bodies
    pub fn manhattan_distance_to(&self, other: &Body<T>) -> T {
        let distance_x = self.x - other.x;
        let distance_y = self.y - other.y;
        distance_x.abs() + distance_y.abs()
    }

    /// Calculates the absolute maximun between two points.
    pub fn chebyshev_distance_to(&self, other: &Body<T>) -> T {
        let distance_x = self.x - other.x;
        let distance_y = self.y - other.y;
        distance_x.abs().max(distance_y.abs())
    }

    /// A function to see if a point is close to another (always try to use EPILSON f32 )
    /// Something like this: "a.is_close_to(&b, f32::EPILSON)"
    pub fn is_close_to(&self, other: &Body<T>, tolerance: T) -> bool {
        (self.x - other.x).abs() <= tolerance && (self.y - other.y).abs() <= tolerance
    }
}
