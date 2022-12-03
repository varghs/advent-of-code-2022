fn part_1(v: &Vec<&str>) -> i32 {
    let (mut total, mut half_size, mut s1, mut s2, mut breaking): (i32, usize, &str, &str, bool);
    total = 0;
    breaking = false;

    for s in v {
        half_size = s.chars().count()/ 2;
        s1 = &s[..half_size];
        s2 = &s[half_size..];
        for c1 in s1.chars() {
            for c2 in s2.chars() {
                if c1 == c2 {
                    if c1.is_lowercase() {
                        total += (c1 as i32) - 96;
                    } else {
                        total += (c1 as i32) - 38;
                    }
                    breaking = true;
                    break;
                }
            }
            if breaking {
                break;
            }
        }
        breaking = false;
    }
    total
}

fn part_2(v: &Vec<&str>) -> i32 {
    let (mut total, mut s1, mut s2, mut s3, mut breaking): (i32, &str, &str, &str, bool);
    total = 0;
    breaking = false;

    for i in (0..v.len()).step_by(3) {
        s1 = v.get(i).unwrap();
        s2 = v.get(i+1).unwrap();
        s3 = v.get(i+2).unwrap();
        for c1 in s1.chars() {
            for c2 in s2.chars() {
                for c3 in s3.chars() {
                    if c1 == c2 && c2 == c3 {
                        if c1.is_lowercase() {
                            total += (c1 as i32) - 96;
                        } else {
                            total += (c1 as i32) - 38;
                        }
                        breaking = true;
                        break;
                    }
                }
                if breaking {
                    break;
                }
            }
            if breaking {
                break;
            }
        }
        breaking = false;
    }
    total
} 


fn main() -> anyhow::Result<()> {
    let v: Vec<&str> = include_str!("input.txt").lines().collect();
    println!("Part 1: {}", part_1(&v));
    println!("Part 2: {}", part_2(&v));
    Ok(())
}
