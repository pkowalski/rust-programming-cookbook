use std::thread;
use std::time::Duration;

fn start_no_shared_data_thread() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        println!("Waiting for 3 seconds");
        thread::sleep(Duration::from_secs(3));
        println!("Done");
    })
}

fn start_shared_data_thread(num: i32, num_vec: Vec<i32>) -> thread::JoinHandle<Vec<i32>> {
    thread::spawn(move || {
        print!("[");
        for i in num_vec.iter() {
            print!("{}", i);
        }
        print!("]");
        println!(" A number from inside the thread: {}", num);
        num_vec
    })
}

fn main() {
    let num = 42;
    let num_vec = vec![1,2,3,4,5];
    let no_move_thread = start_no_shared_data_thread();
    let move_thread = start_shared_data_thread(num, num_vec);

    for _ in 0..10 {
        print!(":");
    }

    println!("Waiting for the thread to finish ... {:?}", 
    no_move_thread.join());
    println!("We can still use a Copy-enabled type: {}", num); 
    println!("Waiting for the thread to finish ... {:?}", 
    move_thread.join());
}
