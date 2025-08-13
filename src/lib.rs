use std::time::{Instant};

pub struct B {
    current_iterations: u32,
    target_iterations: u32,

    suite_start_instant: Option<Instant>,
    suite_end_instant: Option<Instant>,
    iteration_instants: Vec<Instant>,
}

impl B {
    fn new(target_iterations: u32) -> B {
        B {
            current_iterations: 0,
            target_iterations,
            suite_start_instant: None,
            suite_end_instant: None,
            iteration_instants: vec![]
        }
    }

    pub fn n(&mut self) -> bool {
        let instant = Instant::now();
        self.iteration_instants.push(instant);
        match self.current_iterations {
            0 => self.suite_start_instant = Some(instant),
            t if t == self.target_iterations => self.suite_end_instant = Some(instant),
            _ => ()
        }

        self.current_iterations += 1;

        println!("iterations: {} out of {}", self.current_iterations, self.target_iterations);
        self.current_iterations <= self.target_iterations
    }
}

pub fn suite<R, O>(name: &str, mut routine: R)
    where
        R : FnMut(&mut B) -> O,
{
    println!("bench {name}");

    let mut b  = B::new(16);

    routine(&mut b);

    match (b.suite_start_instant, b.suite_end_instant) {
        (Some(s), Some(e)) => {
            println!("Total elapsed: {:?}", e.duration_since(s));
        },
        _ => ()
    }
}
