const fn transform_part(part: usize) -> usize {
    if part == 1 {
        1
    } else {
        3
    }
}

struct MaxSet<T: Ord> {
    max_len: usize,
    set: Vec<T>,
}

impl<T: Ord> MaxSet<T> {
    fn new(max_len: usize) -> Self {
        Self {
            max_len,
            set: Vec::with_capacity(max_len),
        }
    }

    fn add(&mut self, value: T) {
        if self.set.len() < self.max_len {
            self.set.push(value);
        } else {
            let min = self.set.iter_mut().min().unwrap();
            if *min < value {
                *min = value;
            }
        }
    }

    fn with_add(mut self, value: T) -> Self {
        self.add(value);
        self
    }
}

pub fn run<const PART: usize>(input: &str) -> String {
    let num_bests = transform_part(PART);
    struct Acc {
        part: usize,
        bests: MaxSet<usize>,
    }
    impl Acc {
        fn new(num_bests: usize) -> Self {
            Self {
                part: 0,
                bests: MaxSet::new(num_bests),
            }
        }

        fn add_best(self) -> Self {
            Self {
                part: 0,
                bests: self.bests.with_add(self.part),
            }
        }
    }
    format!(
        "{}",
        input
            .lines()
            .fold(Acc::new(num_bests), |acc: Acc, line| {
                let line = line.trim_end_matches("\r\n");
                if line.is_empty() {
                    acc.add_best()
                } else {
                    Acc {
                        part: acc.part + line.parse::<usize>().unwrap(),
                        bests: acc.bests,
                    }
                }
            })
            .add_best()
            .bests
            .set
            .into_iter()
            .sum::<usize>()
    )
}
