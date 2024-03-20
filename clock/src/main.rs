use clock::Clock;

fn main() {
    let clock = Clock::new(0, 120);
    println!("{}", clock.to_string());

    let c = clock.add_minutes(-120);

    println!("{}", c.to_string())
}
