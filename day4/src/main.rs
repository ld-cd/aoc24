use memchr::memchr_iter;
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

#[inline]
fn maybe_xmas(loc: (isize, isize), dir: (isize, isize), chars: &[u8], mat: &[&[u8]]) -> bool {
    match chars.len() {
        0 => true,
        _ => {
            let newloc: (usize, usize) = (
                (loc.0 + dir.0)
                    .try_into()
                    .unwrap_or(isize::MAX.try_into().unwrap()),
                (loc.1 + dir.1)
                    .try_into()
                    .unwrap_or(isize::MAX.try_into().unwrap()),
            );
            if let Some(Some(c)) = mat.get(newloc.0).map(|s| (*s).get(newloc.1)) {
                if *c == chars[0] {
                    maybe_xmas(
                        (newloc.0.try_into().unwrap(), newloc.1.try_into().unwrap()),
                        dir,
                        &chars[1..],
                        mat,
                    )
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

fn run(s: &str) -> (i32, i32) {
    let xmas = "MAS".as_bytes();
    let xsam = "SAM".as_bytes();
    let mat = s.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let mut a = 0;
    let mut b = 0;
    for dir in [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ] {
        for (i, line) in mat.iter().enumerate() {
            for j in memchr_iter(b'X', line) {
                if maybe_xmas(
                    (i.try_into().unwrap(), j.try_into().unwrap()),
                    dir,
                    xmas,
                    mat.as_slice(),
                ) {
                    a += 1
                }
            }
        }
    }

    for (dira, dirb) in [((1, 1), (1, -1))] {
        for (i, line) in mat.iter().enumerate() {
            for j in memchr_iter(b'A', line) {
                let i: isize = i.try_into().unwrap();
                let j: isize = j.try_into().unwrap();
                if (maybe_xmas((i - 2 * dira.0, j - 2 * dira.1), dira, xmas, mat.as_slice())
                    || maybe_xmas((i - 2 * dira.0, j - 2 * dira.1), dira, xsam, mat.as_slice()))
                    && (maybe_xmas((i - 2 * dirb.0, j - 2 * dirb.1), dirb, xmas, mat.as_slice())
                        || maybe_xmas((i - 2 * dirb.0, j - 2 * dirb.1), dirb, xsam, mat.as_slice()))
                {
                    b += 1
                }
            }
        }
    }
    (a, b)
}
