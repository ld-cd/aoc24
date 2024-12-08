use std::collections::{HashMap, HashSet};
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let s = fs::read_to_string("./input.txt")?;
    println!("{:?}", run(&s));

    let start = std::time::SystemTime::now();
    for _ in 0..1024 {
        run(&s);
    }
    let stop = std::time::SystemTime::now();
    println!("{:?}", stop.duration_since(start).unwrap() / 1024);
    Ok(())
}

fn run(s: &str) -> (i64, i64) {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    let mut antinodes_res: HashSet<(isize, isize)> = HashSet::new();
    let mut antennas: HashMap<u8, Vec<(isize, isize)>> = HashMap::new();

    let width = s.lines().next().unwrap().len() as isize;
    let height = s.lines().count() as isize;

    for (j, l) in s.lines().enumerate() {
        for (i, c) in l.as_bytes().iter().enumerate() {
            if *c != b'.' {
                if !antennas.contains_key(c) {
                    antennas.insert(*c, Vec::with_capacity(16));
                }
                antennas.get_mut(c).unwrap().push((i as isize, j as isize));
            }
        }
    }

    for (&c, v) in antennas.iter() {
        for a in 0..(v.len() - 1) {
            for b in a + 1..v.len() {
                let a = v[a];
                let b = v[b];
                let p = (b.0 - a.0, b.1 - a.1);

                let a1 = (b.0 + p.0, b.1 + p.1);
                let a2 = (a.0 - p.0, a.1 - p.1);

                antinodes.insert(a1);
                antinodes.insert(a2);

                let mut march = a;
                while march.0 >= 0 && march.1 >= 0 && march.0 < width && march.1 < height {
                    antinodes_res.insert(march);
                    march = (march.0 + p.0, march.1 + p.1);
                }

                let mut march = a;
                while march.0 >= 0 && march.1 >= 0 && march.0 < width && march.1 < height {
                    antinodes_res.insert(march);
                    march = (march.0 - p.0, march.1 - p.1);
                }
            }
        }
    }

    // for j in 0..height {
    //     for i in 0..width {
    //         if antinodes.contains(&(i, j)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    (
        antinodes
            .iter()
            .filter(|&&a| (a.0 >= 0) && (a.1 >= 0) && (a.0 < width) && (a.1 < height))
            .count() as i64,
        antinodes_res.len() as i64,
    )
}
