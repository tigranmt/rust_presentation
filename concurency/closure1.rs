

fn main()
{
    let out_of_scope = 44; 

    {
        let closure ={  //default: borrow
            println!("Value of out_of_scope inside closure is {}", out_of_scope);
        };

    }

    println!("Value of out_of_scope out of closure is {}", out_of_scope);
}