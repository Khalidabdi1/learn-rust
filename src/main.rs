
fn main() {
// Structs

//tuple
// let o:(&str,i32)=("khalid",10);


//sturcts
struct Book{
name:String,
age:i32
}




let mut userone:User=User{
    active:true,
    name:"khalid".to_string(),
    email:"E@gmail.com".to_string()
};

userone.name="abdi".to_string();

println!("{}",userone.name)










}

struct User{
    active:bool,
    name:String,
    email:String
}
