mod point;
mod bounds;

use point::{Point};
use bounds::{Bounds};

struct Quadtree {
	child_quads: Vec<Box<Quadtree>>,
	quad: Bounds,
	points: Vec<Point>
}

impl Quadtree {

	fn new(&self, corners: Bounds) -> Quadtree {
		Quadtree { child_quads: vec![], quad: corners}
	}

	fn subdivide(&self) -> Vec<Box<Quadtree>> {
		let upper_right : Point = self.bounds.max;
		let mid : Point = Point::new(upper_right.x / 2, upper_right.y / 2);
		let lower_left_box : Bounds = Bounds::new(self.bounds.min, mid);
		let upper_right_box : Bounds = Bounds::ew(mid, self.bounds.max);
		let upper_left_box : Bounds = Bounds::new(Point::new(0, mid.y), Point::new(mid.x, upper_right.y));
		let lower_right_box : Bounds = Bounds::new(Point::new(mid.x, 0), Point::new(upper_right.x, mid.y));

		vec![Box::new(Quadtree::new(lower_left_box)), Box::new(Quadtree::new(upper_left_box)), Box::new(Quadtree::new(upper_right_box)), 
			Box::new(Quadtree::new(lower_right_box))]
	}

	fn add_point(&self, p: Point) {

		children = self.subdivide();

		contained = false;
		contained_by = &self;

		for child in children.iter_mut() {
			match child.contains(p) {
				true => {
					if contained {
						break;
					} else {
						contained = true;
					}
				},
				false => _
			}
		}


		if contained {
			owner: Quadtree = *contained_by;
			owner.add_point(p);
		} else {
			self.points.add(p);
		}
 	}
}

#[test]
fn it_works() {
}
