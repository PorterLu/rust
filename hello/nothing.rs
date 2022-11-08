fn make_nothing() -> () {
    return ();
}

// 返回类型隐含为 ()
fn make_nothing2() {
    // 如果没有指定返回值，这个函数将会返回 ()
}

fn main() {
    let a = make_nothing();
    let b = make_nothing2();

    // 打印a和b的debug字符串，因为很难去打印空
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);
}

