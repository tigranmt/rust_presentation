
// A reference can not outlife its referent
/*fn main()
{
    let _r;
    
    let x = 5;
    _r = &x;   

    println!("r: {}", _r);
}*/




/*
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self 
    { 
        &*self 
    }

    fn share(&mut self) {}
}

fn main() {
   let mut foo = Foo;   
   
   let mut loan = foo.mutate_and_share();
   
   foo.share();
  
}
*/



/*fn main() {
    let vec = Vec::new();

    let mut filled = fill_vec(vec);
   
    println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);

    filled.push(88);

    println!("{} has length {} content `{:?}`", "filled", filled.len(), filled);

}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut v = vec;

    v.push(22);
    v.push(44);
    v.push(66);

    v
}*/


