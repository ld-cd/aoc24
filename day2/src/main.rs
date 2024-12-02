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
fn safe(report: &[i32], ignoredex: Option<i64>) -> Option<i64> {
    let (start, rest) = report.split_first().unwrap();
    if ignoredex.is_some_and(|i| i <= 1) {
        if safe(rest, None).is_none() {
            return None;
        }
    }
    let c = match ignoredex {
        Some(0) => report[2],
        _ => report[1],
    };
    if c > *start {
        let mut last = start;
        for (i, v) in rest.iter().enumerate() {
            if i as i64 == ignoredex.unwrap_or(i64::MAX) {
                continue;
            }
            if *v - *last > 3 || *v <= *last {
                return Some(i as i64);
            }
            last = v;
        }
    } else if c < *start {
        let mut last = start;
        for (i, v) in rest.iter().enumerate() {
            if i as i64 == ignoredex.unwrap_or(i64::MAX) {
                continue;
            }
            if *last - *v > 3 || *last <= *v {
                return Some(i as i64);
            }
            last = v;
        }
    } else {
        return Some(-1);
    }
    None
}

fn run(s: &str) -> (i32, i32) {
    let mut reports: Vec<Vec<i32>> = vec![];
    for l in s.lines() {
        reports.push(
            l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
        )
    }
    let reports = reports.as_slice();
    let mut safes = 0;
    let mut fixedes = 0;
    for report in reports {
        match safe(report, None) {
            Some(i) => {
                if safe(report, Some(i - 1)).is_none()
                    || safe(report, Some(i)).is_none()
                    || safe(report, Some(i + 1)).is_none()
                {
                    fixedes += 1
                }
            }
            None => safes += 1,
        }
    }
    (safes, safes + fixedes)
}
