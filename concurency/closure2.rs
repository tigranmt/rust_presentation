

fn main()
{
    let mut out_of_scope = 44; 

    {
        let mut closure =  move || {  //move
            out_of_scope+=1;         //change moved scope local variable
            println!("Value of out_of_scope inside closure is {}", out_of_scope);
        };

        closure(); //invoke closure

    }

    println!("Value of out_of_scope out of closure is {}", out_of_scope);
}

//Output :
//Value of out_of_scope inside closure is 45
//Value of out_of_scope out of closure is 44