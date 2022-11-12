fn main(){
    let a = [1, 2, 3];
    let b = &a;
    println!("{:p}", b);
    let mut c = vec![1, 2, 3];
    let d = &mut c;
    d.push(4);
    println!(":?", d);
    let e = &42;
    assert_eq!(42, *e);
}

//1.首先定义了一个固定长度的数组a
//2.使用借用操作符&可以取到a的内存地址，这种方式不会引起所有权的转移，因为表达式右侧已经经过借用符号变成了一个位置上下文，它只是共享内存地址
//3.声明了c为动态长度的数组，在下一行通过可变引用将地址复制给d，d调用push方法在最后插入4，当然要获取可变引用首先变量必须是可变的
//对于字面量42本身就是一个值表达式，这里会产生一个临时的值
