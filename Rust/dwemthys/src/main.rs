extern crate tcod;
use tcod::{Console, BackgroundFlag, KeyCode};
use tcod::Key::Special as Special;
use std::rand::{thread_rng, Rng};

struct Point {
    x : i32,
    y : i32
}

impl Point {
    fn translate_x(&self, dist : i32) -> Point {
        Point { x : self.x + dist, y : self. y}
    }

    fn translate_y(&self, dist : i32) -> Point {
        Point { x : self.x, y : self.y + dist}
    }

    fn translate(&self, offset : &Point) -> Point {
        self.translate_x(offset.x).translate_y(offset.y)
    }
}

struct Bounds {
    min : Point,
    max : Point
}

enum Contains {
    DoesContain,
    DoesNotContain
}

impl Bounds {
    fn contains(&self, p : Point) -> Contains {
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

struct Character {
    position : Point,
    display_char : char
}


fn render(con: &mut Console, pc : &Character, mobs: &Vec<Character>) {
    con.clear();
    con.put_char(pc.position.x, pc.position.y, pc.display_char, BackgroundFlag::Set);
    for mob in mobs.iter() {
        con.put_char(mob.position.x, mob.position.y, mob.display_char, BackgroundFlag::Set);
    }

    Console::flush();
}

fn wrap(p : &mut Point, window_bounds : &Bounds, offset : &Point) {
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

fn main() { 
    let window_bounds : Bounds = Bounds { min : Point { x: 0, y: 0}, max : Point { x: 80, y : 50 } };
    let mut con = Console::init_root(window_bounds.max.x, window_bounds.max.y, "libtcod Rust tutorial", false);
    let mut exit = false;
    let mut pc : Character = Character { position : Point { x : 40i32, y : 25i32}, 
                                         display_char : '@' };
    let mut mobs = Vec::new();
    mobs.push(Character { position : Point { x : 10i32, y : 10i32 }, display_char : '%' });
    

    render(&mut con, &pc, &mobs);
    while !(Console::window_closed() || exit) {
        
        let keypress = Console::wait_for_keypress(true);

        let mut offset = Point { x : 0, y : 0 };
        match keypress.key {
            Special(KeyCode::Escape) => exit = true,
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

        match window_bounds.contains(pc.position.translate(&offset)) {
            Contains::DoesContain => pc.position = pc.position.translate(&offset),
            Contains::DoesNotContain => wrap(&mut pc.position, &window_bounds, &offset)
        }

        for mob in mobs.iter_mut() {
            let ref mut mob_pos = mob.position;
            let offset_x = (thread_rng().gen_range(0.0f32, 3.0) - 1.0) as i32;
            match window_bounds.contains(mob_pos.translate_x(offset_x)) {
                Contains::DoesContain => mob_pos.x = mob_pos.translate_x(offset_x).x,
                Contains::DoesNotContain => wrap(mob_pos, &window_bounds, &Point { x : offset_x, y : 0})
            }

            let offset_y = (thread_rng().gen_range(0.0f32, 3.0) - 1.0) as i32;
            match window_bounds.contains(mob_pos.translate_y(offset_y)) {
                Contains::DoesContain => mob_pos.y = mob_pos.translate_y(offset_y).y,
                Contains::DoesNotContain => wrap(mob_pos, &window_bounds, &Point { x : 0, y : offset_y})
            }
        }

        render(&mut con, &pc, &mobs);
    } 
}

