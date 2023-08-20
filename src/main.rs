use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Square {
    x: usize,
    y: usize,
    size: usize,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{}|{},{}",
            self.y,
            self.x,
            self.y + self.size - 1,
            self.x + self.size - 1
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
        x: 0,
        y: 0,
        size: 0,
    };
    let mut size: usize = 1;
    let mut x = 0;
    let mut y = 0;

    while y + size <= map.height && x + size <= map.width {
        if check_square(&map, Square { x, y, size }) {
            if size > max_sq.size {
                max_sq = Square {
                    x,
                    y,
                    size,
                };
                //println!("New Square: {}", max_sq);
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

fn check_square(map: &Map, sq: Square) -> bool {
    // println!("{}", pos.y + size);
    for y in sq.y..(sq.y + sq.size) {
        if map.data[y][sq.x..(sq.x + sq.size)].contains("o") {
            return false;
        }
    }
    true
}
