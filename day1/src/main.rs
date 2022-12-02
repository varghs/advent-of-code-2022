fn parse_arr(s: Result<i64, std::num::ParseIntError>) -> i64 {
    match s {
        Ok(n) => n,
        Err(_) => 0,
    }
}

/* Without using vec::sort */
fn biggest_elf(s: &Vec<i64>) -> i64 {
    let mut elf = 0;
    let mut tmp = 0;

    for num in s {
        match num {
            0 => {
                if tmp > elf {
                    elf = tmp;
                }
                tmp = 0;
            },
            _ => tmp += num,
        }
    }
    elf
}

fn biggest_elves(s: &Vec<i64>) -> (i64, i64, i64) {
    let mut elves: Vec<i64> = Vec::new();
    let mut tmp = 0;

    for num in s {
        match num {
            0 => {
                elves.push(tmp);
                tmp = 0;
            },
            _ => tmp += num,
        }
    }
    let l: usize = elves.len();
    elves.sort();
    (*elves.get(l-1).unwrap(), *elves.get(l-2).unwrap(), *elves.get(l-3).unwrap())
}


fn main() -> anyhow::Result<()> {
    let s: Vec<i64> = std::include_str!("input.txt").lines()
        .map(str::parse::<i64>)
        .map(parse_arr)
        .collect();
    println!("Part 1: {}", biggest_elf(&s));
    let (a1, a2, a3) = biggest_elves(&s);
    println!("Part 2: {}", a1 + a2 + a3);
    Ok(())
}
