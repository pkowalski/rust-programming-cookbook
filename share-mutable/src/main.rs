use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum Shade {
    White,
    Black
}


fn new_painter_thread(data: Arc<Mutex<Vec<Shade>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        {
            let mut d = data.lock().unwrap();

            if d.len() > 0 {
                match d[d.len() - 1] {
                    Shade::White => d.push(Shade::Black),
                    Shade::Black => d.push(Shade::White)
                }
            } else {
                d.push(Shade::Black);
            }
            if d.len() > 5 {
                break;
            }
        }
        thread::sleep(Duration::from_secs(1))
    })
}

fn main() {
    let data = Arc::new(Mutex::new(Vec::<Shade>::new()));
    let threads: Vec<thread::JoinHandle<()>> = 
    (0..2)
    .map(|_| new_painter_thread(data.clone()))
    .collect();

    let _: Vec<()> = threads
    .into_iter()
    .map(|t| t.join().unwrap())
    .collect();

    println!("Result {:?}", data);
}
