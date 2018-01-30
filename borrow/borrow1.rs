#[derive(Debug)]
struct Point
{
    x : i32,
    y : i32
}

fn main()
{
    let mut point1 = Point {x:10, y:20}; 
    
    let point2 = &mut point1; //immutable variable to mutable reference !
    point2.x = 2 * point2.x;  //change X
    point2.y = 2 * point2.y;  //change Y

    println!("point1={:?}", point1); //print 1
    println!("point2={:?}", point2); //print 2
    //
   

}