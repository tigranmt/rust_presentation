use std::thread; 
use std::sync::Mutex; 
use std::sync::Arc; 
use std::time;

fn main()
{
    let ten_ms         = time::Duration::from_millis(1); 
    let one_sec        = time::Duration::from_secs(1); 
    let thread_count   = 1_000_000; //one million 
    let mutexed_number = Arc::new(Mutex::new(0));                  //shareable atomic reference of Mutex
  

    for thread_num in 0 .. thread_count
    {   
        let thread_local_mutex = mutexed_number.clone();            //get a reference to Mutex
        thread::spawn(move || { //closure    
            thread::sleep(ten_ms);       
            let mut number = thread_local_mutex.lock().unwrap();    //get variable inside 
            *number += 1;                                           //change its value           
        });
    }

    //infinit loop
    loop {
         thread::sleep(one_sec);     
         let number = mutexed_number.lock().unwrap();
         if *number != thread_count
         {
            println!("Still counting, number is: {}", *number);
         }
         else {
             println!("Got the right number, number is: {}", *number);
             break;
         }
    }
}