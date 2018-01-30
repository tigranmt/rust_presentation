use std::thread;
use std::sync::mpsc; 

#[derive(Debug)]
struct Value {
    tn : usize
}
fn main()
{
    
    let thread_count = 300;   
   
    //let (push, pull) = mpsc::channel(); 
    let (push, pull) = mpsc::sync_channel(1); 
    

    for thread_num in 0 .. thread_count
    {            
        let p = push.clone();    
        thread::spawn(move || { //closure       
                          
            println!("About to send value {}", thread_num);     
            p.send(Value{tn:thread_num});
            println!("Sent value {}", thread_num);         
        });
    }
   

    thread::spawn(move || { //closure                      
        loop {
             let value =  pull.recv().unwrap();
             println!("Get value from {:?} thread",value);

             if value.tn == thread_count-1 {break;}
        }
    }).join();

}

// Output 
// ...
// Get value from 818 thread
// Get value from 820 thread
// ...


