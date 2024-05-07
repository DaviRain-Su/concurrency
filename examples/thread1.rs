use std::thread;
use std::time::Duration;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Msg {
    id: usize,
    value: usize,
}

fn main() -> anyhow::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // create producer thread
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || produce(i, tx));
    }
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("received: {:?}", msg);
        }
        println!("consumer exit");
        // sercet number
        42
    });

    let result = consumer
        .join()
        .map_err(|e| anyhow::anyhow!("thread join error: {:?}", e))?;
    println!("sercet result: {:?}", result);
    println!("waiting for consumer to finish");
    Ok(())
}

fn produce(id: usize, tx: std::sync::mpsc::Sender<Msg>) -> anyhow::Result<()> {
    loop {
        let rand = rand::random::<usize>() % 1000;
        let msg = Msg { id, value: rand };
        tx.send(msg).unwrap();
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        if rand % 5 == 0 {
            println!("producer {} exit", id);
            break;
        }
    }
    Ok(())
}
