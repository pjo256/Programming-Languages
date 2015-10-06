use point::Point;

pub struct Bounds {
    pub min : Point,
    pub max : Point
}

impl Bounds {
    pub fn new(min: Point, max: Point) -> Bounds {
        Bounds { min : min, max : max }
    }

    pub fn contains(&self, p : Point) {
        if p.x >= self.min.x &&
           p.y >= self.min.y &&
           p.x <= self.max.x &&
           p.y <= self.max.y 
        {
            true
        } else {
            false
        }
    }
}