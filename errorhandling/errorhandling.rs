use std::path::Path;
use std::fs::File;
/* 
enum Result<T,E>
{
    Ok(T),    //wrap workable instance, in case of a success 
    Err(E),   //wrap error instance,message, ..etc , in case of a failure 
}

pub enum Option<T> 
{
    None,     //nothing to report 
    Some(T),  //wrap some value
} 
*/ 

fn main()
{
    let file_path = Path::new("intro1.rs");                 // construct Path instance 
    let mut file = match File::open(&file_path)             // invoke Open(..) file and match returning Result<T> in-place
    {
        Ok(file) => file,                                   //return file object

        Err(err) => {                                       //handle and report error
            println!("Error while opening file: {}", err);
            panic!();                                       //raise unhandled exception
        }

    };
}









/*fn main()
{
    let file_path = Path::new("intro1.rs");                
    let mut file = File::open(&file_path).
            expect(&format!("Error while opening file: {:?}", file_path));
    
}*/