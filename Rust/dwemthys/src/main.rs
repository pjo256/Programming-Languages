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


fn render(con: &mut Console, pc : &Point, mobs: &Vec<Point>) {
    con.clear();
    con.put_char(pc.x, pc.y, '@', BackgroundFlag::Set);
    for mob in mobs.iter() {
        con.put_char(mob.x, mob.y, '%', BackgroundFlag::Set);
    }

    Console::flush();
}

fn wrap(p : &Point, window_bounds : &Bounds, offset : &Point) -> Point {
    let mut pnpc = *p;
    if offset.x > 0 {
        pnpc.x = (pnpc.x + offset.x) % window_bounds.max.x;
    } else if offset.x < 0 {
       pnpc.x = { if pnpc.x > 0 { pnpc.x + offset.x } else { window_bounds.max.x } };
    }

    if offset.y > 0 {
        pnpc.y = (pnpc.y + offset.y) % window_bounds.max.y;
    } else if offset.y < 0 {
        pnpc.y = { if pnpc.y > 0 { pnpc.y + offset.y } else { window_bounds.max.y } };
    }

    return pnpc;
}

fn main() { 
    let window_bounds : Bounds = Bounds { min : Point { x: 0, y: 0}, max : Point { x: 80, y : 50 } };
    let mut con = Console::init_root(window_bounds.max.x, window_bounds.max.y, "libtcod Rust tutorial", false);
    let mut exit = false;
    let mut pc : Point = Point { x : 40i32, y : 25i32};
    let mut mobs = Vec::new();
    mobs.push(Point { x : 10i32, y : 10i32 });
    

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

        match window_bounds.contains(pc.translate(&offset)) {
            Contains::DoesContain => pc = pc.translate(&offset),
            Contains::DoesNotContain => pc = wrap(&pc, &window_bounds, &offset)
        }

        for mob in mobs.iter_mut() {
            let offset_x = (thread_rng().gen_range(0.0f32, 3.0) - 1.0) as i32;
            let mut mob_pos = *mob;
            match window_bounds.contains(mob_pos.translate_x(offset_x)) {
                Contains::DoesContain => mob_pos = mob_pos.translate_x(offset_x),
                Contains::DoesNotContain => mob_pos = wrap(&mob_pos, &window_bounds, &Point { x : offset_x, y : 0})
            }

            let offset_y = (thread_rng().gen_range(0.0f32, 3.0) - 1.0) as i32;
            match window_bounds.contains(mob_pos.translate_y(offset_y)) {
                Contains::DoesContain => mob_pos = mob_pos.translate_y(offset_y),
                Contains::DoesNotContain => mob_pos = wrap(&mob_pos, &window_bounds, &Point { x : 0, y : offset_y})
            }
        }

        render(&mut con, &pc, &mobs);
    } 
}

