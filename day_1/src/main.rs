use std::io;
use std::io::BufRead;
use std::collections::VecDeque;

fn main() -> io::Result<()> {
    let window_len = 3;  // It was 1 for the first star, 3 for the second star.

    let mut history : VecDeque<i64> = VecDeque::new();
    let mut increases : i64 = 0;
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines();
    for line_text in lines {
        let value : i64 = str::parse(&line_text?).unwrap();
        history.push_front(value);
        if history.len() >= window_len + 1 {
            let first : i64 = history.range(0..window_len).sum();
            let second : i64 = history.range(1..(window_len + 1)).sum();
            if first > second {
                increases += 1;
            }
        }
        history.truncate(window_len + 1)
    }
    println!("{} increases", increases);
    Ok(())
}
