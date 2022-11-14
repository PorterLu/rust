fn return_str<'a>() -> &'a str{
    let mut s = "Rust".to_string();
    for i in 0..3{
        s.push_str(" Good");
    }

    &s[..]
}

fn main(){
    let x = return_str();
}

//没有参数仍然有返回值
