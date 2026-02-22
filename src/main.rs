use std::io;

fn suites_fibonacci(n: u32) -> u64 {
    let mut valuer_initial=0;
    let mut valuer_precedent=1;
    for _ in 0..n {
        let somme = valuer_initial + valuer_precedent;
        valuer_initial = valuer_precedent;
        valuer_precedent = somme;
    }
    valuer_initial
}

fn main() -> io::Result<()> {
    println!("Entrée le nombre (n):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let entrer = input.trim_end();
    let n: u32 = entrer.parse().unwrap();
    let fibs = suites_fibonacci(n);
    println!("Le {} ème nombre de Fibonacci est : {}", n   , fibs);
    Ok(())
}
