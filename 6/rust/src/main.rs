//use std::fs;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let part = 1; //part 1 / 2 / 3(both)
    let samp = 1; //sample 1 / 0

    let cur_path = env::current_dir()?;

    let mut sf = "/sample.txt";
    if samp == 0 {
        sf = "/input.txt";
    }
    let file_path = cur_path.parent().expect("path failed").display().to_string().clone() + &sf;

    println!("----- {}", file_path);

    let mut input_file = File::open(file_path)?;
    let mut input_text = String::new();
    input_file.read_to_string(&mut input_text)?;

    let mut vec: Vec<i32> = Vec::new();
    for i in input_text.split("\n") {
        if i.len() > 0 {
            vec.push(i.parse().unwrap());
        }
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

fn part_one(v: Vec<i32>) {
    let mut inc = 0;
    for i in 0..(v.len() - 1) {
        let j = v[i];
        if j < v[i + 1] {
            inc += 1;
        }
    }
    println!("Part 1: {}", inc);
}

fn part_two(v: Vec<i32>) {
    let mut inc = 0;
    for i in 0..(v.len() - 3) {
        let s1 = v[i] + v[i + 1] + v[i + 2];
        let s2 = v[i + 1] + v[i + 2] + v[i + 3];
        if s1 < s2 {
            inc += 1;
        }
    }
    println!("Part 2: {}", inc);
}
