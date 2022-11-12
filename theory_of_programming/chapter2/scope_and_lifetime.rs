fn main(){
    let v = "hello world!";
    assert_eq!(v, "hello world!");
    let v = "hello rust!";
    assert_eq!(v, "hello Rust!");
    {
        let v = "hello World!";
        assert_eq!(v, "hello World!");
    }
    assert_eq!(v, "hello Rust!");
}

//花括号的词法作用域结束后，变量的声明周期就结束了
