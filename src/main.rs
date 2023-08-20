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

    for y in 0..map.height {
        for x in 0..map.width {
            let max_size = std::cmp::min(map.width - x, map.height - y);

            for size in (1..=max_size).rev() {
                let square = Square { x, y, size };
                if check_square(&map, &square) {
                    if size > max_sq.size {
                        max_sq = square;
                    }
                    break;
                }
            }
        }
    }
    max_sq
}

fn check_square(map: &Map, sq: &Square) -> bool {
    // println!("{}", pos.y + size);
    for y in sq.y..(sq.y + sq.size) {
        if map.data[y][sq.x..(sq.x + sq.size)].contains('o') {
            return false;
        }
    }
    true
}
