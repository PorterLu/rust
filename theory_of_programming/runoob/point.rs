struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T{
        &self.x
    }
}

fn main(){
    let p = Point{ x:1, y:2};
    println!("p.x = {}", p.x());
}
