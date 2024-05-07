use concurrency::metrics::Metrics;
use rand::Rng;
use std::thread;

const N: usize = 10;
const M: usize = 5;

fn main() -> anyhow::Result<()> {
    let metrics = Metrics::default();

    println!("{}", metrics);

    for i in 0..N {
        task_worker(i, metrics.clone())?; // Arc::clone(&metrics)
    }

    for _ in 0..M {
        request_worker(metrics.clone()); // Arc::clone(&metrics)
    }

    loop {
        thread::sleep(std::time::Duration::from_secs(5));
        println!("{}", metrics);
    }
}

fn task_worker(id: usize, metrics: Metrics) -> anyhow::Result<()> {
    thread::spawn(move || {
        loop {
            // do long term stuff
            let mut rng = rand::thread_rng();

            thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{}", id)).unwrap();
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(metrics: Metrics) {
    thread::spawn(move || loop {
        // do long term stuff
        let mut rng = rand::thread_rng();

        thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(1..256);
        metrics.inc(format!("req.page.{}", page)).unwrap();
    });
}
