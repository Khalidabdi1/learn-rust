
fn main() {
//rules of ownership

// each value has owner 
    let s1:String=String::from("khalid");
    let two=calc(&s1);

    println!("the length for khalid {}",two);

    // only one owner
    let s2:String=String::from("abdi");
    let s3=s2;
    println!(" is{}",s3);

    // out scope value will drop
    let s4:String=String::from("rust");


    let len:usize=calc(&s4);


 
}


fn calc(s:&String) ->usize{
    s.len()
}

