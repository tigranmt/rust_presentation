fn concat_vec<'a, 'b>(vec1: &'a mut Vec<i32>, vec2: &'b Vec<i32>) -> &'a Vec<i32>
{
    for v2 in vec2
    {
        vec1.push(*v2);
    }

    vec1
    
}

fn main() 
{
   let result;
   let mut v1 = vec![1,2,3,4,5];
   {
        let v2 = vec![6,7,8,9,10];
        result = concat_vec(&mut v1, &v2);
   }
    
   println!("v1 = {:?}", v1); 
}
