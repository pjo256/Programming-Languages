extern crate tcod;
extern crate rand;
use tcod::BackgroundFlag;
use tcod::console::{Console, Root};
use tcod::input::{KeyCode, KeyState};
use tcod::input::Key::Special as Special;
use rand::Rng;

struct Game {
    exit : bool,
    window_bounds : Bounds
}

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

impl Character {
    fn new(x : i32, y: i32, dc: char) -> Character {
        Character { position: Point { x : x, y : y}, display_char: dc }
    }
}

struct NPC {
    position : Point,
    display_char : char
}

impl NPC {
    fn new(x : i32, y: i32, dc: char) -> NPC {
        NPC { position: Point { x : x, y : y }, display_char: dc }
    }
}

trait Update {
    fn update(&mut self, KeyState, &mut Game);
    fn render(&self, &mut Root);
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
            Contains::DoesNotContain => wrap(&mut self.position, &game.window_bounds, &offset)
        }
    }

    fn render(&self, console: &mut Root) {
        console.put_char(self.position.x, self.position.y, self.display_char, 
                         BackgroundFlag::Set);
    }
}
impl Update for NPC {
    fn update(&mut self, keypress: KeyState, game: &mut Game) {
        let mut rng = rand::thread_rng();
        let offset_x = (rng.gen_range(0.0f32, 3.0) - 1.0) as i32;
        match game.window_bounds.contains(self.position.translate_x(offset_x)) {
            Contains::DoesContain => self.position = self.position.translate_x(offset_x),
            Contains::DoesNotContain => wrap(&mut self.position, &game.window_bounds, &Point { x : offset_x, y : 0})
        }

        let offset_y = (rng.gen_range(0.0f32, 3.0) - 1.0) as i32;
        match game.window_bounds.contains(self.position.translate_y(offset_y)) {
            Contains::DoesContain => self.position = self.position.translate_y(offset_y),
            Contains::DoesNotContain => wrap(&mut self.position, &game.window_bounds, &Point { x : 0, y : offset_y})
        }
    }

    fn render(&self, console: &mut Root) {
        console.put_char(self.position.x, self.position.y, self.display_char, 
                         BackgroundFlag::Set);
    }
}


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
    //let mut what = objects;
    //what.push(Box::new(Character::new(40i32, 25i32, '@')) as Box<Update>);
    //what.push(Box::new(NPC::new(10i32, 10i32, '%')) as Box<Update>);
    
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

