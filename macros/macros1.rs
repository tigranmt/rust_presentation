/*
*   :block => (some; another)                   /* something in braces */
*   :expr  => 1, x+1, if {..} else {}..         /*expression*/
*   :item  => fn main() {...}, const a, ..      /*functions, declarations .. */  
*   :meta  => #[a], #[do(Debug)], ..            /*attributes and parameters inside them*/
*   :tt    =>  { a: if x == 2 {..}else {...}}   /* token _tree_ */
*   :ty    =>  i32, u64, String...              /*types*/
*   ... 
*   ...
*/


macro_rules! sum_all {
    ( $( $x:expr ),* ) => { //expression matcher 
        {           
            let mut sum = 0;
            $(
                sum+=$x;
            )*              //expander
            sum
        }
    };
}


fn main()
{ 
    let sum_xs = sum_all!(1,2,3,4,5);
    let sum_ys = sum_all!(6,7,8,9,10);
    println!("sum xs = {}",sum_xs);
    println!("sum ys = {}",sum_ys);
}



