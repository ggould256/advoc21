use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Ixy(i64, i64);

fn line_to_ixy(line: &str) -> io::Result<Ixy> {
    let v = line.split_whitespace().take(2).collect::<Vec<&str>>();

    if let [verb, value_text] = &v[..] {
        let value : i64 = str::parse(&value_text).unwrap();
        match *verb {
            "forward" => Ok(Ixy(value, 0)),
            "back" => Ok(Ixy(-value, 0)),
            "up" => Ok(Ixy(0, -value)),
            "down" => Ok(Ixy(0, value)),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "unrecognized verb"))
        }
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Not two words in line"))
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines();
    let mut position = Ixy(0, 0);

    let aim_mode : bool = true;  // false for one-star solution, true for two-star solution
    let mut aim : i64 = 0;

    for line_text in lines {
        let Ixy(x, y) = position;
        let Ixy(dx, dy) = line_to_ixy(&line_text?)?;
        if aim_mode {
            aim += dy;
            let new_y = y + dx * aim;
            position = Ixy(x + dx, new_y);
            println!("x {} y {} aim {}", x + dx, new_y, aim);
        } else {
            position = Ixy(x + dx, y + dy);
        }
    }

    let Ixy(x, y) = position;
    println!("position is {}, {}", x, y);
    println!("answer is {}", x * y);
    Ok(())
}
