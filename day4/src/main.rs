fn part_1(v: &Vec<&str>) -> i32 {
    let mut c = 0;
    let (mut n, mut n1, mut n2, mut n1_beg, mut n1_end, mut n2_beg, mut n2_end): (Vec<&str>, Vec<&str>, Vec<&str>, i32, i32, i32, i32);
    for s in v {
        n = s.split(",").collect();
        n1 = n.get(0).unwrap().split("-").collect();
        n2 = n.get(1).unwrap().split("-").collect();
        n1_beg = n1.get(0).unwrap().parse::<i32>().unwrap();
        n1_end = n1.get(1).unwrap().parse::<i32>().unwrap();
        n2_beg = n2.get(0).unwrap().parse::<i32>().unwrap();
        n2_end = n2.get(1).unwrap().parse::<i32>().unwrap();
        if (n1_beg >= n2_beg && n1_end <= n2_end) || (n2_beg >= n1_beg && n2_end <= n1_end) {
            c += 1;
        }
    }
    c   
}
fn part_2(v: &Vec<&str>) -> i32 {
    let mut c = 0;
    let (mut n, mut n1, mut n2, mut n1_beg, mut n1_end, mut n2_beg, mut n2_end): (Vec<&str>, Vec<&str>, Vec<&str>, i32, i32, i32, i32);
    for s in v {
        n = s.split(",").collect();
        n1 = n.get(0).unwrap().split("-").collect();
        n2 = n.get(1).unwrap().split("-").collect();
        n1_beg = n1.get(0).unwrap().parse::<i32>().unwrap();
        n1_end = n1.get(1).unwrap().parse::<i32>().unwrap();
        n2_beg = n2.get(0).unwrap().parse::<i32>().unwrap();
        n2_end = n2.get(1).unwrap().parse::<i32>().unwrap();
        if n1_beg <= n2_end && n2_beg <= n1_end {
            c += 1;
        }
    }
    c   
}
fn main() {
    let s: Vec<&str> = include_str!("input.txt").lines().collect();
    println!("Part 1: {}", part_1(&s));
    println!("Part 2: {}", part_2(&s));
}
