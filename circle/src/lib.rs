#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(&self) -> f64 {
        self.radius.powi(2) * std::f64::consts::PI
    }
    pub fn intersect(&self, o: &Circle) -> bool {
        self.center.distance(&o.center) < self.radius + o.radius
    }
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Self {
            center: Point {x, y },
            radius: r
        }
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, p: &Point) -> f64 {
        ((self.x - p.x).powi(2) + (self.y - p.y).powi(2)).sqrt()
    }
}