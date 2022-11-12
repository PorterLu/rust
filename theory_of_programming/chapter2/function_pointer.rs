//在rust中函数是一等公民，所以函数本身就可以作为参数的参数和返回值

pub fn r#match(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32{
    op(a, b)
}

fn sum(a: i32, b:i32)->i32{
    a + b
}

fn product(a:i32, b:i32)->i32{
    a * b
}

fn main(){
    let a = 2;
    let b = 3;
    assert_eq!(r#match(sum, a, b), 5);
    assert_eq!(r#match(product, a, b), 6);
}
