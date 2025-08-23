use std::time::Duration;

fn do_work() {
    // imitate heavy calculations
    std::thread::sleep(Duration::from_secs_f32(0.01));
}

fn main() {
    nbench::suite("abc", |b| {
        // okay to do expensive data set up here
        // start up time is measured separately from iterations

        while b.n() {
            // calls to b.n() measure time between iterations
            // b.n() returns true until nbench collects enough data for
            // reliable measures
            do_work();
        }

        // clean up time is measured too
    });
}