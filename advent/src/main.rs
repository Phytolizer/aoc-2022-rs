fn main() {
    println!("Hello, world!");
}

mod day01;

#[cfg(test)]
mod tests {
    use advent_macros::advent_test;

    #[advent_test(1)]
    fn something_else(input: &str, part: usize) {
        let simple = ["1", "2"];
        let full = ["3", "4"];
        #[advent_magic]
        magic(simple, full)
    }
}
