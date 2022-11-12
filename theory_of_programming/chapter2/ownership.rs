//当位置表达式出现在值上下文中，该位置表达式会把内存地址转一个另外一个位置，这其实就是
//所有权的转移
fn main(){
    let place1 = "hello";
    let place2 = "hello".to_string();
    let other = place1;
    println!("{:?}", place1);
    let other = place2;
    println!("{:?}", place2);       //error
}

//String类型没有实现copy属性
