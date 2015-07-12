extern crate tcod;

use tcod::input::{KeyState};
use window::{Point, Contains, Bounds, Game};
use tcod::console::{Console, Root};

pub trait Update {
    fn update(&mut self, KeyState, &mut Game);
    fn render(&self, &mut Root);
}

pub fn wrap(p : &mut Point, window_bounds : &Bounds, offset : &Point) {
	if offset.x > 0 {
	    p.x = (p.x + offset.x) % window_bounds.max.x;
	} else if offset.x < 0 {
	   p.x = { if p.x > 0 { p.x + offset.x } else { window_bounds.max.x } };
	}

	if offset.y > 0 {
	    p.y = (p.y + offset.y) % window_bounds.max.y;
	} else if offset.y < 0 {
	    p.y = { if p.y > 0 { p.y + offset.y } else { window_bounds.max.y } };
	}
}