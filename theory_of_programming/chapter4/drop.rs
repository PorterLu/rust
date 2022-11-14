use std::ops::Drop;
#[derive(Debug)]
struct S(i32);
impl Drop for S{
    fn drop(&mut self){
        println!("drop {}", self.0);
    }
}

fn print(i: S) -> () {
    println!("{}", i.0);
}

fn main(){
    let x = S(1);
    let z = x;
    print(z);
    {
        let y = S(2);
    }
}
