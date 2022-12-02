fn str_to_tuple(s: &str) -> (&str, &str) {
    let t: Vec<&str> = s.split(' ').collect();
    (t.get(0).unwrap(), t.get(1).unwrap())
    
}

fn move_score(s: &str) -> u16 {
    match s {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn games_results_p1(t: (&str, &str)) -> u16 {
    match t {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        _ => 0,
    }
}

fn games_results_p2<'a>(t: (&'a str, &'a str)) -> &'a str {
    match t {
        ("A", "X") => "Z",
        ("A", "Y") => "X",
        ("A", "Z") => "Y",
        ("B", "X") => "X",
        ("B", "Y") => "Y",
        ("B", "Z") => "Z",
        ("C", "X") => "Y",
        ("C", "Y") => "Z",
        ("C", "Z") => "X",
        _ => "",
    }
}


fn strategy_guide_p1(s: &Vec<(&str, &str)>) -> u16 {
    let mut score = 0;
    for n in s {
        score += move_score(n.1) + games_results_p1(*n);
    }
    score
}

fn strategy_guide_p2(s: &Vec<(&str, &str)>) -> u16 {
    let mut score = 0;
    let mut game_move;
    for n in s { 
        game_move = games_results_p2(*n);
        score += move_score(game_move) + games_results_p1((n.0, game_move));
    }
    score
}

fn main() -> anyhow::Result<()> {
    let s: Vec<(&str, &str)> = include_str!("input.txt")
        .lines()
        .map(str_to_tuple)
        .collect();
    println!("Part 1: {}", strategy_guide_p1(&s));
    println!("Part 2: {}", strategy_guide_p2(&s));
    Ok(())
}
