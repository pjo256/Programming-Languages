extern crate tcod;

use update;
use update::Update as Update;
use window::{Point, Contains, Bounds, Game};
use tcod::BackgroundFlag;
use tcod::console::{Console, Root};
use tcod::input::{KeyCode, KeyState};
use tcod::input::Key::Special as Special;

pub struct Character {
    pub position : Point,
    pub display_char : char
}

impl Character {
    pub fn new(x : i32, y: i32, dc: char) -> Character {
        Character { position: Point { x : x, y : y}, display_char: dc }
    }
}

impl Update for Character {
    fn update(&mut self, keypress: KeyState, game: &mut Game) {
        let mut offset = Point { x : 0, y : 0 };
        match keypress.key {
            Special(KeyCode::Escape) => game.exit = true,
            Special(KeyCode::Up) => {
                offset.y = -1;
            },
            Special(KeyCode::Down) => {
                offset.y = 1;
            },
            Special(KeyCode::Left) => {
                offset.x = -1;
            },
            Special(KeyCode::Right) => {
                offset.x = 1;
            },
            _ => {}
        }

        match game.window_bounds.contains(self.position.translate(&offset)) {
            Contains::DoesContain => self.position = self.position.translate(&offset),
            Contains::DoesNotContain => update::wrap(&mut self.position, &game.window_bounds, &offset)
        }
    }

    fn render(&self, console: &mut Root) {
        console.put_char(self.position.x, self.position.y, self.display_char, 
                         BackgroundFlag::Set);
    }
}