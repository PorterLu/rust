fn main(){
    let mut x = String::from("HelloWorld");
    let mut y = &mut x[..5];
    let mut z = &mut x[5..];
    println!("{} {}", y, z);
}
