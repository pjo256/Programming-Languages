extern crate tcod;
use tcod::{Console, BackgroundFlag, KeyCode, Key};


fn render(con: &mut Console) {
    con.clear();
    con.put_char(40, 25, '@', BackgroundFlag::Set);
    Console::flush();
}

fn main() { 
    let mut con = Console::init_root(80, 50, "libtcod Rust tutorial", false);
    let mut exit = false;
    render(&mut con);
    while !(Console::window_closed() || exit) {
        
        let keypress = Console::wait_for_keypress(true);

        match keypress.key {
            Key::Special(KeyCode::Escape) => exit = true,
            _ => {}
        }

        render(&mut con);
    } 
}

