
fn main() 
{
    let v = vec![1,2,3,4,5,6,7,8,9,0];
    let sum : i32 = v.iter()
                    .skip(4)
                    .take(2)
                    .fold(0, |acc, &i| acc + i );

    println!("sum = {}", sum);
}