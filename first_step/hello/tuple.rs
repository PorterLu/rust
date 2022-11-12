fn swap(a: i32, b: i32) -> (i32, i32){
    return (b, a);
}

fn main(){
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1 );

    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}
