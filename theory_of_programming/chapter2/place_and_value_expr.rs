pub fn temp() -> i32{
    return 1;
}

fn main(){
    let x = &temp();
    temp() = *x;        //错误temp()是值表达式无法放置位置上下文中
}
