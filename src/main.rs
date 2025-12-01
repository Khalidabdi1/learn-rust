fn main() {
    //array
    let mut x :i32=0;
    let arr:[i32;4]=[1,2,3,4];
    let _ =arr.map(|e:i32|{
        println!("{:?}",e);
        x+=e;
    });

    let fruits:[&str;3]=["apple","banana","orange"];
    println!("the fruits is {}",fruits[0]);
    println!("this is arr {:?}",arr);
    println!("res is {}",x);

    let mut human:(String,i32,bool) =("khalid".to_string(),30,true);
    println!("{}",human.0);
    human.0="Abdi".to_string();
        println!("{}",human.0);
    // tuples
    let obj:(&str,i32,[i32;4],bool)=("khalid",30,[3,414,24,4],true);
    println!("{}",obj.2[2]);

    //slice
    let se:&[i32]=&[1,2,3,5,3];

        println!("{:?}",se);
let arr3:&[i32]=&[1,2,3,4,5,6,7];
let sli:&[i32]=&arr3[2..4];
    println!("{:?}",sli);


    //string

    let mut stone:String=String::from("khalids");
    println!("{}",stone);
    stone.push_str(" abdi");
        println!("{}",stone);

//str

let st:String=String::from("hello ,world");
let so:&str=&st[0..4];
println!("{}",so);

p();
 
}


fn p(){
    println!("test p")
}
