use std::io;
use std::io::BufRead;

fn boolify(c : char) -> bool {
    match c {
        '0' => false,
        '1' => true,
        _ => panic!("unrecognized char")
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines();

    let mut one_counts : Vec<u64> = vec![];
    let mut num_lines : u64 = 0;

    for line_text in lines {
        let line_text = line_text?;
        let len = line_text.len();
        let bools = Vec::from_iter(line_text.chars().map(boolify));
        one_counts.resize(len, 0);
        for i in 0..len {
            if bools[i] { one_counts[i] += 1; }
        }
        num_lines += 1;
    }

    let majorities = Vec::from_iter(
        one_counts.iter().map(|count| (count * 2) > num_lines)
    );
    let mut gamma : u64 = 0;
    let mut epsilon : u64 = 0;
    for b in majorities {
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        if b { gamma += 1; }
        else { epsilon += 1; }
    }

    println!("gamma {}", gamma);
    println!("epsilon {}", epsilon);
    println!("power {}", gamma * epsilon);
    Ok(())
}
