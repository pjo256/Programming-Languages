extern crate tcod;
use tcod::{Console, BackgroundFlag, KeyCode};
use tcod::Key::Special as Special;
use std::rand;


fn render(con: &mut Console, pos_x: i32, pos_y: i32, mobs: &Vec<(i32, i32)>) {
    con.clear();
    con.put_char(pos_x, pos_y, '@', BackgroundFlag::Set);
    for mob in mobs.iter() {
        con.put_char(mob.0, mob.1, '%', BackgroundFlag::Set);
    }

    Console::flush();
}

fn main() { 
    let con_x = 80i32;
    let con_y = 50i32;
    let mut con = Console::init_root(con_x, con_y, "libtcod Rust tutorial", false);
    let mut exit = false;
    let mut pos_x = 40i32;
    let mut pos_y = 25i32;
    let mut mobs = Vec::new();
    mobs.push((10i32, 10i32));

    render(&mut con, pos_x, pos_y, &mobs);
    while !(Console::window_closed() || exit) {
        
        let keypress = Console::wait_for_keypress(true);

        match keypress.key {
            Special(KeyCode::Escape) => exit = true,
            Special(KeyCode::Up) => pos_y = {
                if pos_y > 0 { pos_y - 1 } else { con_y }
            },
            Special(KeyCode::Down) => pos_y = (pos_y + 1) % con_y,
            Special(KeyCode::Left) => pos_x = {
                if pos_x > 0 { pos_x - 1 } else { con_x }
            },
            Special(KeyCode::Right) => pos_x = (pos_x + 1) % con_x,
            _ => {}
        }

        for mob in mobs.iter() {
            let offset_x = rand::thread_rng().gen_range(0.0f32, 3.0) - 1.0 as i32;
            if (mob.0 + offset_x) > 0 && (mob.0 + offset_x) < (con_x - 1) {
                mob.0 += offset_x;
            }

            let offset_y = rand::thread_rng().gen_range(0.0f32, 3.0) - 1.0 as i32;
            if (mob.1 + offset_y) > 0 && (mob.1 + offset_y) < (con_y - 1) {
                mob.1 += offset_y;
            }
        }

        render(&mut con, pos_x, pos_y, &mobs);
    } 
}

