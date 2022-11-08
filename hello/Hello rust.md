## Hello rust

### println!

​	这是`rust`中的宏，使用这个宏可以进行输出，我们可以有如下的代码

```rust
fn main(){
	println!("{} days", 31);
}
```

​	这样用一个类似`scala`形式`printf`中的用`{}`指定一个输出的占位符。同时使用数字指定不同的变量，或者直接用变量的名字。

```rust
fn main(){
    println!("{0}, this is {1}", Alice, Bob);
    println!("{subject} {verb} {object}",
    		object="the lazy dog”,
    		subject="fox",
        	verb="jump over");
}
```

​	同样像C语言一样可以指定输出的格式，二进制、十进制、十六进制，填充的功能。

```rust
fn main(){
    println!("Base 2: 	{:b}", 8);
    println!("{number:0>width$}", number=1, width=5);
}
```

​	这样输出就可以指定是否填充，对齐方式等等。

### 推测

​	rust大部分的情况都可以自动推测类型：

```rust
fn main() {
    // rust 推断出x的类型
    let x = 13;
    println!("{}", x);

    // rust也可以显式声明类型
    let x: f64 = 3.14159;
    println!("{}", x);

    // rust 也支持先声明后初始化，但很少这样做
    let x;
    x = 0;
    println!("{}", x);
}
```

### 可变类型和不可变类型

​	像`scala`中一样，rust中也分为可变和不可变的类型，对于可变的类型我们用`mut`进行修饰。

```rust
fn main(){
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
}
```

### 常见类型

```rust
fn main(){
    let x = 12;	//默认的情况是整型
    let a = 12u8;
    let b = 4.3; //默认情况是浮点数
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello rust!";
    println!("{} {}	{} {} {} {} {} {}",
       	x, a, b, c, bv, t.0, t.1, sentence");
}
```

* 布尔型：`bool`表示`true` or `false`。
* 无符号整型: `u8`、`u32`、`u64`、`u128`表示正整数。

* 有符号数：`i8`、`i32`、`i64`、`i128`表示正负数。
* 指针大小的整数: `usize`、`isize`表示内存中内容的索引和大小
* 浮点：`f32`和`f64`
* 元组: `(value, value, ...)`， 用于在栈上传递参数
* 数组：在编译时已经知道长度的相同元素的集合
* 切片：将要求放宽至运行时
* str：运行时知道长度的文本。

### 类型转换

​	我们不能想当然地进行不同类型地运算，我们需要进行类型转换。

```rust
fn main(){
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);
}
```

### 常量

​	常量运行我们定义一个在代码中多次被用到的公共值，常量必须有显示的类型

```rust
const PI: f32 = 3.14159;

fn main() {
    println!(
        "To make an apple {} from scratch, you must first create a universe.",
        PI
    );
}
```

### 数组

```rust
fn main(){
    let nums: [i32; 3] = [1, 2, 3];
    println!{"{:?}", nums};
    println!("{}", nums[1]);
}
```

​	这里的`{:?}`相当于进行了一次遍历，我们不可以直接`println`出一个数组，使用这种方法相当于遍历整个数组进行输出。

### 函数

```rust
fn add(x: i32, y: i32) -> i32{
    return x + y;
}

fn main(){
    println!("{}", add(42, 13));
}
```

​	我们还可以通过元组来返回多个值。

```rust
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn main() {
    // 返回一个元组
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // 将元组解构为两个变量
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}
```

​	使用空元组表示没有返回值，当然不写的话，也是不返回值。

```rust
fn make_nothing() -> (){
    return ();
}

fn make_nothing2(){}

fn main(){
    let a = make_nothing();
    let b = make_nothing2();
    println!("{:?} {:?}", a, b);
}
    
}
```

