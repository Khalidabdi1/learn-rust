
fn main() {
//refernece
 let mut account=bank{
    owner:"khalid".to_string(),
    blance:23.2
 };

 account.check_blance();
 account.withdraw(45.50);
 account.check_blance();


}


struct bank{
    owner:String,
    blance:f64
}

impl bank {
    fn withdraw(&mut self ,amount:f64){
println!(" withdrawing {} from account owned by {}",amount,self.owner);
self.blance -=amount;
    }


    fn check_blance(&self){
        println!("Account owned by {} has {}",self.owner,self.blance);
    }
}




