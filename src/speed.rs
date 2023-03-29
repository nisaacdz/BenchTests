use std::time;

pub struct BenchTester {
    rounds: u32,
}

impl BenchTester {
    pub fn new(rounds: u32) -> Self {
        Self { rounds }
    }
    pub fn test<F: Fn()>(&self, f: F) -> time::Duration {
        let time = time::Instant::now();
        for _ in 0..self.rounds {
            f();
        }
        let ans = time.elapsed() / self.rounds;
        ans
    }

    pub fn bench<F: Fn()>(&self, f: F) {
        let d = self.test(f);
        println!("Finished, Ellapsed millis = {}", d.as_millis())
    }
}
