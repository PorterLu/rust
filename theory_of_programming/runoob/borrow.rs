fn main(){
    let mut x = String::from("String");
    let mut y = &mut x;
    let z = &mut y;
    println!("{} {} {}", x, y, z);
}
