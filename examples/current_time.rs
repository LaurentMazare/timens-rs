extern crate anyhow;

const NB_ITERS: usize = 10_000_000;

fn main() -> anyhow::Result<()> {
    let now = timens::Time::now();
    println!("start: {:?}", now);
    for _i in 0..100 {
        let _now = timens::Time::now();
    }
    let start = timens::Time::now();
    for _i in 1..NB_ITERS {
        let _now = timens::Time::now();
    }
    let dt = timens::Time::now() - start;
    println!("dt: {:?}, per-iter: {:?}", dt, dt / (NB_ITERS as f64));
    Ok(())
}
