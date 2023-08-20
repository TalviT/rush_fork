use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Point {
    x: usize,
    y: usize,
}

struct Square {
    pos: Point,
    size: usize,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{}|{},{}",
            self.pos.y,
            self.pos.x,
            self.pos.y + self.size,
            self.pos.x + self.size
        )
    }
}

struct Map {
    width: usize,
    height: usize,
    data: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(Path::new(&args[1])).unwrap();
    let filebuf = BufReader::new(&file);
    let mut map = Map {
        width: 0,
        height: 0,
        data: Vec::new(),
    };
    map.data = filebuf.lines().collect::<Result<_, _>>().unwrap();
    map.height = map.data.len();
    map.width = map.data[0].len();
    println!("{}", search_square(map));
}

fn search_square(map: Map) -> Square {
    let mut max_sq = Square {
        pos: Point { x: 0, y: 0 },
        size: 0,
    };
    let mut size: usize = 1;
    let mut x = 0;
    let mut y = 0;

    while y + size < map.height {
        if check_square(&map, Point { x, y }, size) {
            if size > max_sq.size {
                max_sq = Square {
                    pos: Point { x, y },
                    size: size - 1,
                }; // @note size or size - 1??
                println!("New Square: {}", max_sq);
                size += 1;
            }
        } else if x + size >= map.width {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }
    }
    max_sq
}

fn check_square(map: &Map, pos: Point, size: usize) -> bool {
    println!("{}", pos.y + size);
    for y in pos.y..(pos.y + size) {
        if map.data[y][pos.x..(pos.x + size)].contains("o") {
            return false;
        }
    }
    true
}
