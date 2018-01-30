/*Interior, Exterior mutability  */


#[derive(Debug)]
struct Loss 
{
    gu : f32,
    gr : f32,
}

fn interior_mut()
{
    let mut v = vec![Loss {gu:10.0, gr:5.0},Loss {gu:20.0, gr:10.0}, //has to be defined _mut_ as well
                        Loss {gu:30.0, gr:15.0},Loss {gu:40.0, gr:20.0}];

    {
        let _one = v.last_mut(); //interior mutability 
        _one.unwrap().gu = 33.0; //ok, can reassign, interior mutability
        //_one = v.first_mut(); //Error, NO exterior mutability !
    }
    
    println!("[3]={:?}", v[3]);
}

fn exterior_mut() 
{
    let v = vec![Loss {gu:10.0, gr:5.0},Loss {gu:20.0, gr:10.0},
                        Loss {gu:30.0, gr:15.0},Loss {gu:40.0, gr:20.0}];

    let mut _one = v.last(); //exterior mutability 
    _one = v.first(); //ok, can reassign, exterior mutability
    //_one.unwrap().gu = 333.0; //Error, NO interior mutability !
}

fn main() 
{
    exterior_mut(); 
    interior_mut();
}