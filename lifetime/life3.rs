
struct Foo
{
    val : i32
}

impl Foo {
    
    fn mutate(&mut self) -> &Foo 
    {
        self.val = 333333;
        &*self 
    }
    fn share(&self) { println!("Foo is {}", self.val);}
}

fn main() 
{
   let mut foo = Foo {val:-44444}; //create an object
   let aftermut = foo.mutate();    //assign new value 
   foo.share();                    //share 
}


