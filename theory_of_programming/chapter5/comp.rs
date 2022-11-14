fn main(){
    let a = [1, 2, 3, 4];
    let b = [2, 3, 4];
    assert_eq!(&a[1..=3], &b);
}
