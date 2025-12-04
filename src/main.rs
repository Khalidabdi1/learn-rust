

fn main() {
// collections

let mut _list:Vec<String>=Vec::new();
_list.push("khalid".to_string());
let mut  _t:Vec<i32>=vec![1,2,3,4,5];
_t.push(100);

for i in _t.iter(){
    println!("{}",i)
}

println!("the list is {:?}",_list);


let arr:Vec<i32>=vec![1,2,3,4,5];

let re:&i32=&arr[4];
let find=arr.get(0);

match find {
    Some(t)=> println!(" the find is {t}"),
    None =>println!("not found")
}

let arr:i32=arr[2];



println!("the 3 is {} and 4 is {} ",arr,re)


}





