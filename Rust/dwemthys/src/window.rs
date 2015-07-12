pub struct Game {
    pub exit : bool,
    pub window_bounds : Bounds
}

pub struct Point {
    pub x : i32,
    pub y : i32
}


impl Point {
    pub fn translate_x(&self, dist : i32) -> Point {
        Point { x : self.x + dist, y : self. y }
    }

    pub fn translate_y(&self, dist : i32) -> Point {
        Point { x : self.x, y : self.y + dist }
    }

    pub fn translate(&self, offset : &Point) -> Point {
        self.translate_x(offset.x).translate_y(offset.y)
    }
}

pub struct Bounds {
    pub min : Point,
    pub max : Point
}

pub enum Contains {
    DoesContain,
    DoesNotContain
}

impl Bounds {
    pub fn contains(&self, p : Point) -> Contains {
        if p.x >= self.min.x &&
           p.y >= self.min.y &&
           p.x <= self.max.x &&
           p.y <= self.max.y 
        {
            Contains::DoesContain
        } else {
            Contains::DoesNotContain
        }
    }
}
