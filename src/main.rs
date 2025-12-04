

fn main() {
// error handling

let res =de(10.0,0.0 );

match res{
    Ok(x)=>println!("res is {}",x),
    Err(e)=>println!("error is {}",e)
}




}


// enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }



// enum Option<T>{
//     Some(T),
//     None
// }


fn de(numer:f64,deno:f64) ->Result<f64,String>{
    if deno==0.0{
    Err("not allow".to_string())
    }else {
        Ok(numer/deno)
    }
}


