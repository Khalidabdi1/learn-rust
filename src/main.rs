
fn main() {
   
 let s =sum(5,5);
 println!("{}",s);
 p();
 
//block
 let x ={
    let price=5;
    let q=10;
   price * q  
 };

 println!("{}",x);


 println!(" the num is {}",sub(5, 5));
 println!(" the bmi is {:.2}",BMI(70.0, 1.82));

 
}


fn BMI(heighkg:f64,heightm:f64)->f64{
    heighkg / (heightm *heightm)
}


fn sub(a:i32,v:i32)->i32{
    a-v

}


fn sum(a:i32,b:i32)->i64{
    (a+b)
.into()
}

fn p(){
    println!("hello world")
}
