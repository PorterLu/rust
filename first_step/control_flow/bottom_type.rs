#![feature(never_type)]
fn foo() -> !{
    loop{ println!("jn"); }
}

fn main(){
    let i = if true {
        foo();
    } else {
        100
    };
    assert_eq!(i, 100);
}
