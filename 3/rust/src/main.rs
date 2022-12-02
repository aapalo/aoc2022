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
        //println!("{}: {}", d, d.chars().nth(2).unwrap());
    }
    //println!("{}", vec!["X", "Y"][1]);

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

fn part_one(v: Vec<(char,char)>) {
    let mut points = 0;
    for i in 0..(v.len()) {
        points += i;
        //println!("Round {}: {} vs {}, {} points", i, v[i].0, v[i].1, points);
    }
    println!("Part 1: {}", points);
}


fn part_two(v: Vec<(char,char)>) {
    let mut points = 0;
    for i in 0..(v.len()) {
        points += i;
        //println!("Round {}: {} vs {}, {} points", i, v[i].0, v[i].1, points);
    }
    println!("Part 2: {}", points);
}
