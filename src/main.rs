use std::time::Duration;
use std::{collections::VecDeque, thread, thread::sleep};
use std::sync::{Arc,Mutex, Condvar};
struct BoundedQueue<T> {
    queue: VecDeque<T>,
    capacity: usize,
    condvar: Condvar,

}

impl<T> BoundedQueue<T> {
    fn new(capacity: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            capacity,
            condvar: Condvar::new()
        }
    }

    fn push(&mut self, element: T) -> Result<(), &str> {
        // Push only if there is room.
        if self.queue.len() < self.capacity {
            self.queue.push_back(element);
            return Ok(());
        }
        return Err("Pushed to full queue");
    }

    fn pop(&mut self) -> Option<T> {
        return self.queue.pop_front();
    }
}

const N_PRODUCTION: i32 = 100;
const N_CAPACITY: usize = 100;
fn main() {
    // Goal: To implement the solution to the consumer - producer problem.
    // Queue can hold up to 10 elements at a time.
    // Produced will produce 20 elements and put them into the queue.
    // Consumer will consume 20 elements from the queue.
    // Neither should access the queue while the other accesses it.
    // Produced can not add to full queue.
    // Consumer can not pop from empty queue.
    // Producer and consumer should run in separate threads.
    let queue = Arc::new(Mutex::new(BoundedQueue::<i32>::new(N_CAPACITY)));
    let push_queue = queue.clone();
    
    let push_handle = thread::spawn(move || {
        print!("Producer started!\n");
        let mut i = 0;
        while i < N_PRODUCTION {
            
            sleep(Duration::from_millis(3));
            match push_queue.lock().unwrap().push(i) {
                Ok(()) => {
                    i+=1;
                    //print!("{}| Pushed {}\n", i, i);
                },
                Err(s) => {
                        print!(">>> {}|{} <<<\n", i, s);
                        continue;
                    },
            }
        }
    });
    let pull_queue = queue.clone();
    #[allow(unused_variables)]                                              // TODO: Remove this.
    let pull_handle = thread::spawn(move || {
        print!("Consumer started!\n");
        let mut j = 0;
        while j < N_PRODUCTION {
            sleep(Duration::from_millis(0));
            match pull_queue.lock().unwrap().pop() {
                Some(v) => {
                                    j+=1;
                                    //print!("{}| Consumed {}\n", j, v);
                                },
                None => continue,//print!(">>> {}| Tried to consume from empty buffer <<<\n", j),
            }
        }
    });

    match push_handle.join() {
        Ok(_) => print!("\nProducer finished!\n"),
        Err(_) => print!("\n>>> Producer panicked! <<<\n"),
    }

    match pull_handle.join() {
        Ok(_) => print!("Consumer finished!\n"),
        Err(_) => print!(">>> Consumer panicked! <<<\n"),
    }
    let final_size = queue.lock().unwrap().queue.len(); 
    match final_size{
        0usize => println!("\nBounded queue is empty!\n"),
        _ => println!("\n>>> Bounded queue has {}elements left! <<<\n", final_size)
    }
}