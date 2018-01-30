use std::rc::Rc;

fn another_reference(re:&Rc<i32>)
{
   let another_cloned = re.clone(); //new reference to the _same_ memory , count + 1 = 3   
   println!("Count of [another_cloned] owners: {}", Rc::strong_count(&another_cloned));
} // goes out of scope, count -1  = 2

fn main()
{
   
    let loss = Rc::new(1000); //heap value
    println!("Count of [loss] owners: {}", Rc::strong_count(&loss)); // count 1
   
    {       
        let mut cloned = loss.clone(); //new reference to the _same_ memory , count + 1 = 2     
        println!("Count of [loss] owners: {}", Rc::strong_count(&loss));

        another_reference(&cloned); //pass a borrow & to function

        println!("Count of [loss] owners: {}", Rc::strong_count(&loss));
    } // goes out of scope, count - 1  = 1

    println!("Count of [loss] owners: {}", Rc::strong_count(&loss));
}