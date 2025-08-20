use std::{time::{Duration, Instant}, u128};

pub struct B {
    current_iterations: usize,
    target_iterations: usize,

    suite_start_instant: Option<Instant>,
    suite_end_instant: Option<Instant>,
    iteration_instants: Vec<Instant>,
}

struct Measures {
    startup: Duration,
    min: Duration,
    max: Duration,
    mean: Duration,
    std_dev: Duration,
    cleanup: Duration,
    total: Duration,
    iterations: usize,
}

impl B {
    fn new(target_iterations: usize) -> B {
        B {
            current_iterations: 0,
            target_iterations,
            suite_start_instant: None,
            suite_end_instant: None,
            iteration_instants: Vec::with_capacity(target_iterations)
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
        self.current_iterations <= self.target_iterations
    }

    fn duration(&self) -> Option<Duration> {
        self.suite_start_instant.zip(self.suite_end_instant).map(|(s, e)| e.duration_since(s))
    }

    fn calc_measures(&self) -> Option<Measures> {
        if self.suite_start_instant.is_none() || self.suite_end_instant.is_none() || self.iteration_instants.is_empty() {
            return None;
        }

        let start = self.suite_start_instant.unwrap();
        let end = self.suite_end_instant.unwrap();
        let start_to_end = end.duration_since(start);
        let instants = &self.iteration_instants;
        
        let startup = instants.first().unwrap().duration_since(start);
        let cleanup = end.duration_since(*instants.last().unwrap());

        let mut min = Duration::MAX;
        let mut max = Duration::default();
        let mut total = Duration::default();

        let starts_slice = self.iteration_instants.get(0..self.iteration_instants.len()-1).unwrap();
        let ends_slice = self.iteration_instants.get(1..self.iteration_instants.len()).unwrap();
        let intervals : Vec<Duration> = starts_slice.iter().zip(ends_slice)
            .map(|(s, e)| e.duration_since(*s))
            .collect();
        let n = intervals.len();

        for i in &intervals {
            min = std::cmp::min(min, *i);
            max = std::cmp::max(max, *i);
            total += *i;
        }

        let mean = total / n.try_into().unwrap();
        let std_dev = Duration::from_nanos(
            (intervals.into_iter().map(|v| (v.as_nanos() - mean.as_nanos()).pow(2)).sum::<u128>() / (n - 1) as u128).isqrt() as u64
        );

        Some(Measures {
            startup,
            min,
            max,
            mean,
            std_dev,
            cleanup,
            total: start_to_end,
            iterations: n
        })
    }
}

pub fn suite<R, O>(name: &str, routine: R)
    where
        R : Fn(&mut B) -> O,
{
    println!("bench {name}");

    let goal = Duration::from_secs(5);


    let mut n : usize = 1;
    let mut b = B::new(n);
    let mut duration = Some(Duration::ZERO);

    while let Some(d) = duration {
        if d > Duration::ZERO {
            n = goal.div_duration_f64(d.div_f64(n as f64)).ceil() as usize;
        }
        if d >= goal {
            break;
        }
        b = B::new(n);
        routine(&mut b);
        duration = b.duration();
    }

    match b.calc_measures() {
        Some(m) => {
            println!("Total: {:?}\nIterations: {:?}\nStartup: {:?}\nCleanup: {:?}\nMin: {:?}\nMax: {:?}\nMean: {:?}\nStdDev: {:?}",
                m.total,
                m.iterations,
                m.startup,
                m.cleanup,
                m.min,
                m.max,
                m.mean,
                m.std_dev
            );
        },
        None => ()
    }
}

