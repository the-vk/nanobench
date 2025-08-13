use std::time::Duration;

fn do_work() {
    // imitate heavy calculations
    std::thread::sleep(Duration::from_secs(1));
}

fn main() {
    nbench::suite("abc", |b| {
        while b.n() {
            do_work();
        }
    });
}