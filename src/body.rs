#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Body {
    x: f64,
    y: f64,
}

impl Body {
    /// Create a new Body(Point)
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Get the X of a Body
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get the Y of a Body
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Calculates de distance between 2 points. this makes the Point A be at 0,0
    pub fn distance_to(&self, other: &Body) -> f64 {
        let distance_x = self.x - other.x;
        let distance_y = self.y - other.y;
        ((distance_x).powi(2) + (distance_y).powi(2)).sqrt()
    }

    /// The relative distance/position
    pub fn relative_to(&self, other: &Body) -> (f64, f64) {
        let relative_x = self.x - other.x;
        let relative_y = self.x - other.x;
        (relative_x, relative_y)
    }

    /// Calculates de distance WITHOUT the sqrt() function
    pub fn distance_squared_to(&self, other: &Body) -> f64 {
        let distance_x = self.x - other.x;
        let distance_y = self.y - other.y;
        (distance_x).powi(2) + (distance_y).powi(2)
    }

    /// Calculates the absolute distance between 2 Bodies
    pub fn manhattan_distance_to(&self, other: &Body) -> f64 {
        let distance_x = self.x - other.x;
        let distance_y = self.y - other.y;
        distance_x.abs() + distance_y.abs()
    }

    /// Calculates the absolute maximun between two points.
    pub fn chebyshev_distance_to(&self, other: &Body) -> f64 {
        let distance_x = self.x - other.x;
        let distance_y = self.y - other.y;
        distance_x.abs().max(distance_y.abs())
    }
}
