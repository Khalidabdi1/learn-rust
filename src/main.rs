use std::string;


fn main() {
// enum



let  user:Gamer=Gamer { 
    name: String::from("khalid")
    , age: 24
    , status:Status::Busy
 };

p(&user);

 fn p(users:&Gamer){
    println!("the user name is {} and the age is {}",users.name,users.age);

    match users.status{
        Status::Online =>println!("is online"),
        Status::Busy =>println!("is bussy"),
        Status::Offline =>println!("is offline")
    }
 }

 user.you();
}




enum Status{
    Online,
    Offline,
    Busy
}


struct Gamer{
    name:String,
    age:u32,
    status:Status
}


impl Gamer{

    fn you(&self){
        println!("hi gemaer {}",self.name)
    }
}




