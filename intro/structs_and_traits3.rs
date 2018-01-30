/// Defines a point of a two i32 values
///
/// # Parameters
///
/// * `x` - A 32 bit x component.
/// * `y` - A 32 bit y component.
#[derive(Debug)]
struct Point
{
    x : i32, 
    y : i32
}



fn main()
{
   let point = Point {x:10, y:10};
  
}





/*impl Drop for Point
{
     fn drop(&mut self) {
        println!("Destructor of x={},y={}", self.x, self.y);
    }
}*/
