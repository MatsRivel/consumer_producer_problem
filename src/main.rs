#[allow(unused_imports)]  
use std::time::Duration;
#[allow(unused_imports)]  
use std::{collections::VecDeque, thread, thread::sleep};
use std::sync::{Arc,Mutex, Condvar};

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

    fn push(&mut self, element: T) -> Option<()> {
        // Push only if there is room.
        match self.is_full(){
            false => {
                self.queue.push_back(element);
                Some(())
            }
            true => None,
        } // No ";", so it returns the output of the match.
    }

    fn pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }
    #[allow(dead_code)] // TODO: Remove
    fn is_empty(&mut self) -> bool{
        self.queue.is_empty()
    }
    fn is_full(&mut self) -> bool{
        self.queue.len() == self.capacity
    }
}

const N_PRODUCTION: i32 = 10;
const N_CAPACITY: usize = 9;
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
    let pull_queue = queue.clone();
    
    let push_handle = thread::spawn(move || {
        let mut i = 0;
        while i < N_PRODUCTION {
            let mut guard = match push_queue.0.lock(){
                Ok(v) => v,                            // Lock succeeded, keep going.
                Err(s) => panic!("{}",s),   // TODO: Handle error better.
            };
            
            match guard.push(i){
                Some(_) => {                             // Push successfull. 
                            push_queue.2.notify_one();   // Notify pull that at least one element is available.
                            println!("Pushed {}\t|",i);
                            i+=1;
                            continue;                    // Jump to the start of the loop again.
                        },   
                None => {},                              // Pushing to full queue. Moving on to wait section.
            }
            push_queue.2.notify_one();                   // Notify pull that at least one element is available.
            
            println!("Push is waiting...");
            match push_queue.1.wait(guard){              // Waiting for notification of available space.
                Ok(_) => {},                             // No issue, go to beginning of loop.           
                Err(s) => panic!("{}",s),   // TODO: Handle error better.
            }
        }
        push_queue.2.notify_all();
        
    });

    let pull_handle = thread::spawn(move || {
        let mut i = 0;
        while i < N_PRODUCTION {
            let mut guard = match pull_queue.0.lock(){
                Ok(v) => v,                            // Lock succeeded, keep going.
                Err(s) => panic!("{}",s),   // TODO: Handle error better.
            };
            
            match guard.pop(){
                Some(v) => {                        // Pull successfull. 
                            pull_queue.1.notify_one();   // Notify push that at least one space is available.
                            println!("\t\t\t|\tPulled {}",v);
                            i+=1;
                            continue;
                        },   
                None => {},                              // Pulling from empty queue. Moves on to wait section
            }
            pull_queue.1.notify_one();                   // Notify push that at least one space is available.
            
            println!("\t\t\t|\tPull is waiting...");
            match pull_queue.2.wait(guard){              // Waiting for notification of available space.
                Ok(_) => {},                             // No issue, go to beginning of loop.                                            
                Err(s) => panic!("{}",s),   // TODO: Handle error better.
            }
        }
        pull_queue.1.notify_all();

    });


    match pull_handle.join() {
        Ok(_) => {
                //print!("Consumer finished!\n");
                },
        Err(_) => print!(">>> Consumer panicked! <<<\n"),
    }
    match push_handle.join() {
        Ok(_) => {
                //print!("\nProducer finished!\n");
                },
        Err(_) => print!("\n>>> Producer panicked! <<<\n"),
    }
    let final_size = queue.0.lock().unwrap().queue.len(); 
    match final_size{
        0usize => {
                //println!("\nBounded queue is empty!\n");
                },
        _ => println!("\n>>> Bounded queue has {}elements left! <<<\n", final_size)
    }
}