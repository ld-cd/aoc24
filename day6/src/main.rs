use memchr::{memchr, memchr_iter};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let s = fs::read_to_string("./input.txt")?;
    println!("{:?}", run(&s));

    let start = std::time::SystemTime::now();
    for _ in 0..32 {
        run(&s);
    }
    let stop = std::time::SystemTime::now();
    println!("{:?}", stop.duration_since(start).unwrap() / 32);
    Ok(())
}

fn run(s: &str) -> (i32, i32) {
    let obs: HashSet<(isize, isize)> = s
        .lines()
        .enumerate()
        .flat_map(|(j, l)| memchr_iter(b'#', l.as_bytes()).map(move |i| (i as isize, j as isize)))
        .collect();

    let mut visited: HashMap<(isize, isize), HashSet<usize>> = HashMap::new();

    let mut width = 0;
    let height = s.lines().count() as isize;

    let mut pos = (0, 0);
    for (j, l) in s.lines().enumerate() {
        if let Some(i) = memchr(b'^', l.as_bytes()) {
            width = l.len() as isize;
            pos = (i as isize, j as isize);
            break;
        }
    }

    let start = pos;

    let mut dir: usize = 0;
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut path = vec![];
    loop {
        match visited.entry(pos) {
            Entry::Vacant(e) => {
                e.insert(HashSet::from([dir % 4]));
            }
            Entry::Occupied(e) => {
                e.into_mut().insert(dir % 4);
            }
        }
        let cd = directions[dir % 4];
        if obs.contains(&(pos.0 + cd.0, pos.1 + cd.1)) {
            dir += 1;
        } else {
            path.push((pos, dir % 4));
            pos = (pos.0 + cd.0, pos.1 + cd.1);
        }
        if pos.0 < 0 || pos.0 >= width || pos.1 < 0 || pos.1 >= height {
            break;
        }
    }

    let mut looping_locs: HashSet<(isize, isize)> = HashSet::new();
    let mut localcache: HashSet<((isize, isize), usize)> = HashSet::with_capacity(128);
    let mut pathcache: HashSet<((isize, isize), usize)> = HashSet::with_capacity(visited.len() * 2);
    let mut vcache: HashSet<(isize, isize)> = HashSet::with_capacity(visited.len());
    // Dumb Method:
    for (i, (loc, d)) in path.iter().enumerate() {
        pathcache.insert((*loc, *d));
        if *loc != start && !vcache.contains(loc) {
            vcache.insert(*loc);
            let (mut pos, mut dir) = path[i - 1];
            dir += 1;
            localcache.drain();
            loop {
                localcache.insert((pos, dir % 4));
                let cd = directions[dir % 4];
                if obs.contains(&(pos.0 + cd.0, pos.1 + cd.1))
                    || loc == &(pos.0 + cd.0, pos.1 + cd.1)
                {
                    dir += 1;
                } else {
                    pos = (pos.0 + cd.0, pos.1 + cd.1);
                }
                if pos.0 < 0 || pos.0 >= width || pos.1 < 0 || pos.1 >= height {
                    break;
                } else if localcache.contains(&(pos, dir % 4))
                    || pathcache.contains(&(pos, dir % 4))
                {
                    looping_locs.insert(*loc);
                    break;
                }
            }
        }
    }

    (visited.len() as i32, looping_locs.len() as i32)
}
