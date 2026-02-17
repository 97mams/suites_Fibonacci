use std::io;

fn suites_fibonacci(n: u32) -> u64 {
    let mut init_values=0;
    let mut next_value=1;
    for _ in 0..n {
        let temp = init_values + next_value;
        init_values = next_value;
        next_value = temp;
    }
    init_values
}

fn main() -> io::Result<()> {
    println!("Please enter your name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim_end();
    let n: u32 = input.parse().unwrap();
    let fibs = suites_fibonacci(n);
    println!("Fibonacci sequence: {:?}", fibs);
    Ok(())
}
