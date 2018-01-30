#[derive(Debug)]
struct Point{
    x : i32,
    y : i32
}

fn scale_point(point : &mut Point, scale: i32) //borrow point as mutable
{
    point.x = scale * point.x;  //change X
    point.y = scale * point.y;  //change Y
}

fn print_point(point: &Point) //borrow point as immutable
{
    println!("point={:?}", point); 
}

fn main()
{
    let mut point = Point {x:10, y:20};  //create mutable variable

    scale_point(&mut point, 2);          //pass as mutable 
    print_point(&point);                 //pass as immutable

}