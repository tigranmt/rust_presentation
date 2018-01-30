fn main()
{
    let mut data = vec![1, 2, 3];
    let x = &mut data[0];       // get reference 
    println!("{}", x);      // use of reference
    data.push(4);           // change !
  
}


