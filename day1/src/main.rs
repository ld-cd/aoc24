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
    let lines = s.lines();
    let (mut left, mut right) = (Vec::<i32>::new(), Vec::<i32>::new());
    for line in lines {
        if line.len() > 2 {
            let s = line.split_whitespace().collect::<Vec<&str>>();
            left.push(i32::from_str_radix(s[0], 10).unwrap());
            right.push(i32::from_str_radix(s[1], 10).unwrap());
        }
    }
    left.sort();
    right.sort();
    let right = right.as_slice();
    let left = left.as_slice();
    let p1 = std::iter::zip(left, right)
        .map(|x| (x.0 - x.1).abs())
        .sum::<i32>();

    let mut rp = 0;
    let mut rs = 0;
    for l in left {
        let mut count = 0;
        loop {
            if rp >= right.len() {
                return (p1, rs);
            }
            if right[rp] < *l {
                rp += 1;
            } else if right[rp] == *l {
                count += 1;
                rp += 1;
            } else {
                rs += l * count;
                break;
            }
        }
    }
    (p1, rs)
}
