use concurrency::metrics::AtomicMap;
use rand::Rng;
use std::thread;

const N: usize = 2;
const M: usize = 4;

fn main() -> anyhow::Result<()> {
    let metrics = AtomicMap::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "req.page.0",
        "req.page.1",
        "req.page.2",
        "req.page.3",
    ]);

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

fn task_worker(id: usize, metrics: AtomicMap) -> anyhow::Result<()> {
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

fn request_worker(metrics: AtomicMap) {
    thread::spawn(move || loop {
        // do long term stuff
        let mut rng = rand::thread_rng();

        thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(0..4);
        metrics.inc(format!("req.page.{}", page)).unwrap();
    });
}
