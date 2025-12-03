
fn main() {
//shadowing
let x:i32= 5;
let x:i32=x+1;
{

    let x:i32=x*2;
    println!("the value is x{}",x);
}

println!("the value of x is {}",x);

//mutable
let mut y:i32=7;
y=10;
println!("the value of x is {}",y);

let space:&str="";
println!("the lenth is {}",space.len())





}


