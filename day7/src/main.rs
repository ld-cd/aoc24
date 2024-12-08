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

struct Line<'a> {
    req: usize,
    numbers: &'a [usize],
}

impl<'a> Line<'a> {
    #[inline]
    fn possible(&self) -> bool {
        if self.numbers.len() == 1 {
            self.numbers[0] == self.req
        } else if self.numbers.len() > 1 {
            let ln = *self.numbers.last().unwrap();
            match self.req % ln {
                0 => {
                    Line {
                        req: self.req / ln,
                        numbers: &self.numbers[..self.numbers.len() - 1],
                    }
                    .possible()
                        || (self.req > ln
                            && (Line {
                                req: self.req - ln,
                                numbers: &self.numbers[..self.numbers.len() - 1],
                            }
                            .possible()))
                }
                _ => {
                    self.req > ln
                        && Line {
                            req: self.req - ln,
                            numbers: &self.numbers[..self.numbers.len() - 1],
                        }
                        .possible()
                }
            }
        } else {
            unreachable!();
        }
    }

    #[inline]
    fn possible_sillyelephant(&self) -> bool {
        if self.numbers.len() == 1 {
            self.numbers[0] == self.req
        } else if self.numbers.len() > 1 {
            let ln = *self.numbers.last().unwrap();
            match self.req % ln {
                0 => {
                    Line {
                        req: self.req / ln,
                        numbers: &self.numbers[..self.numbers.len() - 1],
                    }
                    .possible_sillyelephant()
                        || (self.req > ln
                            && (Line {
                                req: self.req - ln,
                                numbers: &self.numbers[..self.numbers.len() - 1],
                            }
                            .possible_sillyelephant()))
                        || (self.req.ilog10() > ln.ilog10() && {
                            let m = 10usize.pow(ln.ilog10() + 1);
                            self.req % m == ln
                                && Line {
                                    req: self.req / m,
                                    numbers: &self.numbers[..&self.numbers.len() - 1],
                                }
                                .possible_sillyelephant()
                        })
                }
                _ => {
                    (self.req > ln
                        && Line {
                            req: self.req - ln,
                            numbers: &self.numbers[..self.numbers.len() - 1],
                        }
                        .possible_sillyelephant())
                        || (self.req.ilog10() > ln.ilog10() && {
                            let m = 10usize.pow(ln.ilog10() + 1);
                            self.req % m == ln
                                && Line {
                                    req: self.req / m,
                                    numbers: &self.numbers[..&self.numbers.len() - 1],
                                }
                                .possible_sillyelephant()
                        })
                }
            }
        } else {
            unreachable!();
        }
    }
}

fn run(s: &str) -> (usize, usize) {
    let mut lines = Vec::new();
    for l in s.lines() {
        let (req, ns) = l.split_once(':').unwrap();
        let ns: &'static [usize] = Box::new(
            ns.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>(),
        )
        .leak();
        lines.push(Line {
            req: req.parse().unwrap(),
            numbers: ns,
        })
    }
    (
        lines
            .iter()
            .map(|l| if l.possible() { l.req } else { 0 })
            .sum::<usize>(),
        lines
            .iter()
            .map(|l| if l.possible_sillyelephant() { l.req } else { 0 })
            .sum::<usize>(),
    )
}
