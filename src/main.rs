use std::collections::HashMap;



fn main() {
// hash map


let mut scors:HashMap<String,i32> =HashMap::new();
scors.insert(String::from("khalid"),23);
scors.insert(String::from("abdi"),23);
scors.insert(String::from("kalib"),23);


let team=String::from("khalid2");
let age=scors.get(&team).copied().unwrap_or(0);
println!("{:?}",scors);
println!("the age is {}",age);


for(i,m) in &scors{
    println!("the value :{} and key is {}",i,m);
}



}





