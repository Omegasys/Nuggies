use std::time::Instant;

fn fake_install_simulation() {
    for _ in 0..10_000 {
        let _ = "simulate package resolution";
    }
}

#[test]
fn benchmark_install_simulation() {
    let start = Instant::now();

    fake_install_simulation();

    let duration = start.elapsed();

    // Not strict—just ensures it completes reasonably fast
    assert!(duration.as_millis() < 500);
}
