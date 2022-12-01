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

    let mut vec: Vec<i32> = Vec::new();
    for i in input_text.split("\n") {
        if i.len() > 0 {
            vec.push(i.parse().unwrap());
        } else {
            vec.push(-1);
        }
    }
    vec.push(-1);
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
    let mut cals = 0;
    let mut elves: Vec<i32> = Vec::new();
    for i in 0..(v.len() - 1) {
        //println!("{}: {}", i, cals);
        if v[i] > -1 {
            cals += v[i];
        } else {
            elves.push(cals);
            cals = 0;
        }
    }
    elves.push(cals);

    let mut maxcal = 0;
    let mut maxidx = 0;
    for i in 0..(elves.len() - 1) {
        let j = elves[i];
        //println!("{}: {}", i, j);
        if j > maxcal {
            maxcal = j;
            maxidx = i;
        }
    }
    let maxidx2 = 1+ maxidx as i32;
    println!("Part 1: idx {}, cal {}", maxidx2, maxcal);

}

fn part_two(v: Vec<i32>) {
    let mut cals = 0;
    let mut elves: Vec<i32> = Vec::new();
    for i in 0..(v.len() - 1) {
        if v[i] > -1 {
            cals += v[i];
        } else {
            elves.push(cals);
            cals = 0;
        }
    }
    elves.push(cals);

    // sort the list from smallest to biggest
    elves.sort_unstable();

    // print the last item, ie the biggest value (part_one solution!)
    //println!("{}", elves[elves.len()-1]);

    // add up the 3 largest values
    let calsum = elves[elves.len()-1] + elves[elves.len()-2] + elves[elves.len()-3];

    println!("Part 2: {}", calsum);
}
