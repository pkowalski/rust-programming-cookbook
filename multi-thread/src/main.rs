use std::thread;

fn parallel_map(data: Vec<Vec<i32>>) -> Vec<thread::JoinHandle<Vec<i32>>> {
    data.into_iter()
        .map(|chunk| thread::spawn(move || chunk.into_iter().map(|c| c * 2).collect()))
        .collect()
}

fn main() {
    let data = vec![vec![1, 2, 3], vec![4, 4, 5], vec![6, 7, 7]];

    let results: Vec<i32> = parallel_map(data.clone())
    .into_iter()
    .flat_map(|thread| thread.join().unwrap())
    .collect();

    let data: Vec<i32> = data.into_iter().flat_map(|e| e)
     .collect();

     println!("{:?} -> {:?}", data, results);
}
