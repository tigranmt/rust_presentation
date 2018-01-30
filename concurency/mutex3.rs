use std::thread; 
use std::sync::Mutex; 
use std::sync::Arc; 

fn main()
{
    let thread_count    = 100; 
    let mutexed_number  = Arc::new(Mutex::new(0));                  //shareable atomic reference of Mutex
    let mut results     = Vec::with_capacity(thread_count);

    for thread_num in 0 .. thread_count
    {   
        let thread_local_mutex = mutexed_number.clone();            //get a reference to Mutex
        results.push(thread::spawn(move || { //closure           
            let mut number = thread_local_mutex.lock().unwrap();    //get variable inside 
            *number += 1;                                           //change its value
            println!("Data inside {} thread is equal to {}", thread_num, *number);
        }));
    }

    for handle in results
    {
        handle.join(); //wait for completion of all threads
    }

}

//Output: 
//  ...
//  Data inside 3 thread is equal to 4
//  Data inside 15 thread is equal to 5
//  Data inside 16 thread is equal to 6
//  Data inside 6 thread is equal to 7
//  ...



