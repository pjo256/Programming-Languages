extern crate tcod;
extern crate rand;

mod window;
mod update;
mod character;
mod npc;

use character::Character;
use npc::NPC;
use update::Update;
use window::{Point, Contains, Bounds, Game};
use tcod::BackgroundFlag;
use tcod::console::{Console, Root};
use tcod::input::{KeyCode, KeyState};
use tcod::input::Key::Special as Special;
use rand::Rng;

fn render(root: &mut Root, objs: &Vec<Box<Update>>) {
    root.clear();
    for obj in objs.iter() {
        obj.render(root);
    }

    root.flush();
}

fn update(objs: &mut Vec<Box<Update + 'static>>, keypress: KeyState, game: &mut Game) {
    for obj in objs.iter_mut() {
        obj.update(keypress, game);
    }
}

fn main() {     
    let window_bounds : Bounds = Bounds { min : Point { x: 0, y: 0}, max : Point { x: 80, y : 50 } };
    let mut root = Root::initializer()
        .size(window_bounds.max.x, window_bounds.max.y)
        .title("libtcod Rust tutorial") 
        .fullscreen(false)
        .init();

    let mut game : Game = Game { exit : false, window_bounds : window_bounds };
    
    let mut pc = Box::new(Character::new(40i32, 25i32, '@')) as Box<Update>;
    let mut mob = Box::new(NPC::new(10i32, 10i32, '%')) as Box<Update>;
    let mut objects : Vec<Box<Update>> = vec![pc, mob];
 

    render(&mut root, &objects);
    while !(root.window_closed() || game.exit) {
        let keypress = root.wait_for_keypress(true);
        update(&mut objects, keypress, &mut game);
        render(&mut root, &objects);
    } 
}

