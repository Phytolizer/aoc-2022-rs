#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

mod dec01;

#[cfg(test)]
mod tests {
    use advent_macros::advent_test;

    #[advent_test(1)]
    fn dec01() {
        magic(["24000", "45000"], ["69289", "205615"]);
    }
}
