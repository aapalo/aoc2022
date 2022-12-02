//use std::fs;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let part = 3; //part 1 / 2 / 3(both)
    let samp = 0; //sample 1 / 0

    let cur_path = env::current_dir()?;

    let mut sf = "/sample.txt";
    if samp == 0 {
        sf = "/input.txt";
    }
    let file_path = cur_path.parent().expect("path failed").display().to_string().clone() + &sf;

    //println!("----- {}", file_path);

    let mut input_file = File::open(file_path)?;
    let mut input_text = String::new();
    input_file.read_to_string(&mut input_text)?;

    let mut vec: Vec<(char,char)> = Vec::new();
    let data = input_text.lines();
    for d in data {
        vec.push((d.chars().nth(0).unwrap(),d.chars().nth(2).unwrap()));
    }
    
    if part == 1 {
        part_one(vec);
    } else if part == 2 {
        part_two(vec);
    } else {
        part_one(vec.clone());
        part_two(vec.clone());
    }
    Ok(())
}

/*
1   A   X   rock
2   B   Y   paper
3   C   Z   scissors
*/

fn rps(opponent: char, player: char) -> i32{
    let mut p = 0;
    // shape (1,2,3 - R, P, S)
    if player == 'X' {
        p += 1;
    } else if player == 'Y' {
        p += 2;
    } else if player == 'Z' {
        p += 3;
    }
    //outcome (0 loss, 3 draw, 6 win)
    if (opponent == 'A' && player == 'X')
            || (opponent == 'B' && player == 'Y')
            || (opponent == 'C' && player == 'Z') {
        p += 3; // draw
    } else if (opponent == 'A' && player == 'Y')
            || (opponent == 'B' && player == 'Z')
            || (opponent == 'C' && player == 'X') {
        p += 6; // win
    }
    return p;
}

fn part_one(v: Vec<(char,char)>) {
    let mut points = 0;
    for i in 0..(v.len()) {
        points += rps(v[i].0, v[i].1);
        //println!("Round {}: {} vs {}, {} points", i, v[i].0, v[i].1, points);
    }
    println!("Part 1: {}", points);
}


/*
X lose
Y draw
Z win
*/
fn wdl(opponent: char, round: char) -> i32 {
    let mut p = 0;
    if round == 'Y' {
        p += 3; //draw
        if opponent == 'A' {
            p += 1;
        } else if opponent == 'B' {
            p += 2;
        } else if opponent == 'C' {
            p += 3;
        }
    } else if round == 'X' {
        p += 0; //lose
        if opponent == 'A' {
            p += 3;
        } else if opponent == 'B' {
            p += 1;
        } else if opponent == 'C' {
            p += 2;
        }
    } else if round == 'Z' {
        p += 6; //win
        if opponent == 'A' {
            p += 2;
        } else if opponent == 'B' {
            p += 3;
        } else if opponent == 'C' {
            p += 1;
        }
    }
    return p;
}
fn part_two(v: Vec<(char,char)>) {
    let mut points = 0;
    for i in 0..(v.len()) {
        points += wdl(v[i].0,v[i].1);
        //println!("Round {}: {} vs {}, {} points", i, v[i].0, v[i].1, points);
    }
    println!("Part 2: {}", points);
}
