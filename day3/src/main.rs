use memchr::memmem::find_iter;
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

#[derive(Debug)]
enum ParseState {
    FirstEnd(usize),
    SecondEnd(usize, usize),
    Error,
}

fn run(s: &str) -> (i32, i32) {
    let mut a = 0;
    let mut b = 0;
    let mut on = true;

    let mut doiter = find_iter(s.as_bytes(), "do()");
    let mut dontiter = find_iter(s.as_bytes(), "don't()");

    let mut nextdo = doiter.next().unwrap_or(usize::MAX);
    let mut nextdont = dontiter.next().unwrap_or(usize::MAX);
    for mi in find_iter(s.as_bytes(), "mul(") {
        let mut ps = ParseState::FirstEnd(mi);
        match s.as_bytes().get(mi + 4) {
            Some(b'-') => ps = ParseState::FirstEnd(mi + 4),
            Some(c) => {
                if c.is_ascii_digit() {
                    ps = ParseState::FirstEnd(mi + 4)
                } else {
                    continue;
                }
            }
            None => return (a, b),
        }
        while let ParseState::FirstEnd(i) = ps {
            match s.as_bytes().get(i + 1) {
                Some(c) => {
                    if c.is_ascii_digit() {
                        ps = ParseState::FirstEnd(i + 1)
                    } else if *c == b',' {
                        if let Some(c) = s.as_bytes().get(i + 2) {
                            if c.is_ascii_digit() || *c == b'-' {
                                ps = ParseState::SecondEnd(i, i + 2)
                            } else {
                                ps = ParseState::Error;
                            }
                        } else {
                            ps = ParseState::Error;
                        }
                    } else {
                        ps = ParseState::Error;
                    }
                }
                None => return (a, b),
            }
        }
        while let ParseState::SecondEnd(i, j) = ps {
            match s.as_bytes().get(j + 1) {
                Some(b')') => break,
                Some(c) => {
                    if c.is_ascii_digit() {
                        ps = ParseState::SecondEnd(i, j + 1)
                    } else {
                        ps = ParseState::Error
                    }
                }
                None => return (a, b),
            }
        }
        match ps {
            ParseState::FirstEnd(_) => unreachable!(),
            ParseState::Error => continue,
            ParseState::SecondEnd(i, j) => {
                match (
                    String::from_utf8_lossy(&s.as_bytes()[mi + 4..i + 1]).parse::<i32>(),
                    String::from_utf8_lossy(&s.as_bytes()[i + 2..j + 1]).parse::<i32>(),
                ) {
                    (Ok(f), Ok(s)) => {
                        match (mi >= nextdo, mi >= nextdont) {
                            (false, false) => {}
                            (true, false) => {
                                on = true;
                                nextdo = doiter.next().unwrap_or(usize::MAX);
                            }
                            (false, true) => {
                                on = false;
                                nextdont = dontiter.next().unwrap_or(usize::MAX);
                            }
                            (true, true) => {
                                on = nextdo >= nextdont;
                                nextdo = doiter.next().unwrap_or(usize::MAX);
                                nextdont = dontiter.next().unwrap_or(usize::MAX);
                            }
                        }
                        a += f * s;
                        if on {
                            b += f * s;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    (a, b)
}
