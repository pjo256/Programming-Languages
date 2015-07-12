extern crate tcod;
extern crate rand;

use update;
use update::Update as Update;
use window::{Point, Contains, Bounds, Game};
use tcod::BackgroundFlag;
use tcod::console::{Console, Root};
use tcod::input::{KeyCode, KeyState};
use tcod::input::Key::Special as Special;
use rand::Rng;

pub struct NPC {
    pub position : Point,
    pub display_char : char
}

impl NPC {
    pub fn new(x : i32, y: i32, dc: char) -> NPC {
        NPC { position: Point { x : x, y : y }, display_char: dc }
    }
}

impl Update for NPC {
    fn update(&mut self, keypress: KeyState, game: &mut Game) {
        let mut rng = rand::thread_rng();
        let offset_x = (rng.gen_range(-1.0, 1.0) - 1.0) as i32;
        match game.window_bounds.contains(self.position.translate_x(offset_x)) {
            Contains::DoesContain => self.position = self.position.translate_x(offset_x),
            Contains::DoesNotContain => update::wrap(&mut self.position, &game.window_bounds, &Point { x : offset_x, y : 0})
        }

        let offset_y = (rng.gen_range(0.0f32, 3.0) - 1.0) as i32;
        match game.window_bounds.contains(self.position.translate_y(offset_y)) {
            Contains::DoesContain => self.position = self.position.translate_y(offset_y),
            Contains::DoesNotContain => update::wrap(&mut self.position, &game.window_bounds, &Point { x : 0, y : offset_y})
        }
    }

    fn render(&self, console: &mut Root) {
        console.put_char(self.position.x, self.position.y, self.display_char, 
                         BackgroundFlag::Set);
    }
}