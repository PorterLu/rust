fn main(){
    //this is a example of line comment
    
    /*
     * this is another type of comment, a block comment.
     */

    let x = 5 + /*1*/5;
    println!("Hello World! x = {}", x);
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quicker brown fox",
             verb="jumps over");

    println!("Base 10:              {}", 69420);
    println!("Base 2(binary):       {:b}", 69420);
    println!("Base 16(hexadecimal:  {:x}", 69420);

    println!("{number:0>width$}", number=1, width=5);
}
