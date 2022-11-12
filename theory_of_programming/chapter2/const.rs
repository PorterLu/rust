const a: u32 = 3;
static b: u32 = 3;
fn main(){
    println!("{:p}", &a);
    println!("{:p}", &b);
    println!("{:p}", &3);
}
