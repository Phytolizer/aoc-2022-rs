fn main() {
    println!("Hello, world!");
}

mod day01;

#[cfg(test)]
mod tests {
    use advent_macros::advent_test;

    #[advent_test(1)]
    fn something_else() {
        magic(["1", "2"], ["3", "4"]);
    }
}
