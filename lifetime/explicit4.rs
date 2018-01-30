fn foo<'a, 'b>(x: &'a u32, y: &'b u32) -> &'a u32 {
    x
}

fn main() {
    let x = 12;
    let z: &u32 = 
    {
        let y = 42;
        foo(&x, &y)       
    };
}