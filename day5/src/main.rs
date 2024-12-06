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

fn run(s: &str) -> (i32, i32) {
    let mut befores: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut afters: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut p1 = 0;
    let mut p2 = 0;
    for l in s.lines() {
        if l.contains("|") {
            let (b, a) = l.split_once("|").unwrap();
            let (b, a) = (b.parse().unwrap(), a.parse().unwrap());

            if !befores.contains_key(&b) {
                befores.insert(b, [a].into_iter().collect());
            } else {
                befores.get_mut(&b).unwrap().insert(a);
            }

            if !afters.contains_key(&a) {
                afters.insert(a, [b].into_iter().collect());
            } else {
                afters.get_mut(&a).unwrap().insert(b);
            }
        } else if !l.is_empty() {
            let pages: Vec<i32> = l.split(",").map(|s| s.parse().unwrap()).collect();
            let mut visited: HashSet<i32> = HashSet::new();
            let mut bad = false;
            for page in pages.iter() {
                if befores
                    .get(page)
                    .unwrap_or(&HashSet::new())
                    .is_disjoint(&visited)
                {
                    visited.insert(*page);
                } else {
                    bad = true;
                    break;
                }
            }
            if !bad {
                p1 += pages[(pages.len() - 1) / 2];
            } else {
                let mut fixedpages: Vec<i32> = vec![];
                let mut pageset: HashSet<i32> = pages.into_iter().collect();
                while !pageset.is_empty() {
                    let mut ps: Option<i32> = None;
                    for p in pageset.iter() {
                        if pageset.is_disjoint(afters.get(p).unwrap_or(&HashSet::new())) {
                            ps = Some(*p);
                            break;
                        }
                    }
                    if let Some(p) = ps {
                        pageset.remove(&p);
                        fixedpages.push(p);
                    } else {
                        unreachable!();
                    }
                }
                p2 += fixedpages[(fixedpages.len() - 1) / 2];
            }
        }
    }
    (p1, p2)
}
