#[allow(unused_imports)]  
use std::time::Duration;
#[allow(unused_imports)]  
use std::{collections::VecDeque, thread, thread::sleep};
use std::sync::{Arc,Mutex, Condvar};

#[allow(dead_code)]                                              // TODO: Remove this.
struct BoundedQueue<T> {
    queue: VecDeque<T>,
    capacity: usize,

}

impl<T> BoundedQueue<T> {
    fn new(capacity: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            capacity,
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

    fn is_empty(&mut self) -> bool{
        return self.queue.is_empty()
    }

    fn is_full(&mut self) -> bool{
        return self.queue.len() == self.capacity;
    }
}

const N_PRODUCTION: i32 = 10;
const N_CAPACITY: usize = 3;
fn main() {
    // Goal: To implement the solution to the consumer - producer problem.
    // Queue can hold up to 10 elements at a time.
    // Produced will produce 20 elements and put them into the queue.
    // Consumer will consume 20 elements from the queue.
    // Neither should access the queue while the other accesses it.
    // Produced can not add to full queue.
    // Consumer can not pop from empty queue.
    // Producer and consumer should run in separate threads.
    let queue = Arc::new((Mutex::new(BoundedQueue::<i32>::new(N_CAPACITY)), Condvar::new(), Condvar::new()) );
    let push_queue = queue.clone();
    
    let push_handle = thread::spawn(move || {
        //print!("Producer started!\n");
        let mut i = 0;
        while i < N_PRODUCTION {
            sleep(Duration::from_millis(1)); // TODO: REMOVE
            println!("i: {}",i);
            // Try to access: If yes, propagate value to push. Else, reset loop.
            let mut push_lock_result = match push_queue.0.lock(){
                Ok(v) => v,
                Err(e) => {
                    println!(">>> {} <<<", e);
                    continue;
                },
            };



            // Try to push element. If queue has room, push and continue. Else reset loop.
            match push_lock_result.push(i){
                Ok(_) => {
                    i+=1;
                    push_queue.2.notify_all();
                },
                Err(_s) => {
                    print!(">>> i: {}|{} <<<\n", i, _s);
                    push_queue.2.notify_all();
                    match push_queue.1.wait_while(push_queue.0.lock().unwrap(), |push_q: &mut BoundedQueue<i32>| push_q.is_full()){
                        Ok(_) => {}, // Run as normal.
                        Err(_) => continue, // Skip to next iter.
                    }
                    continue;
                },
            }
        }
    });

    let pull_queue = queue.clone();
    let pull_handle = thread::spawn(move || {
        //print!("Consumer started!\n");
        let mut j = 0;
        while j < N_PRODUCTION {
            sleep(Duration::from_millis(10)); // TODO: REMOVE
            // Try to access: If yes, propagate value to push. Else, reset loop.
            println!("j: {}",j);
            let mut pull_lock_result= match pull_queue.0.lock(){
                Ok(v) => v,
                Err(e) => { // Tried to access locked mutex
                    println!(">>> {} <<<", e);
                    continue;
                },
            };

            // Try to push element. If queue has room, push and continue. Else reset loop.
            match pull_lock_result.pop(){
                Some(_) => {
                    j+=1;
                    pull_queue.1.notify_all();
                },
                None => { // Tried to pop from empty queue.
                    println!(">>> j: {}| Pulled from empty queue. <<<", j);
                    pull_queue.1.notify_all();
                    match pull_queue.2.wait_while(pull_queue.0.lock().unwrap(), |push_q: &mut BoundedQueue<i32>| push_q.is_empty()){
                        Ok(_) => {}, // Run as normal.
                        Err(_) => continue, // Skip to next iter.
                    }
                    continue;},
            }
        }
    });

    match push_handle.join() {
        Ok(_) => {
                //print!("\nProducer finished!\n");
                },
        Err(_) => print!("\n>>> Producer panicked! <<<\n"),
    }


    match pull_handle.join() {
        Ok(_) => {
                //print!("Consumer finished!\n");
                },
        Err(_) => print!(">>> Consumer panicked! <<<\n"),
    }
    let final_size = queue.0.lock().unwrap().queue.len(); 
    match final_size{
        0usize => {
                //println!("\nBounded queue is empty!\n");
                },
        _ => println!("\n>>> Bounded queue has {}elements left! <<<\n", final_size)
    }
}