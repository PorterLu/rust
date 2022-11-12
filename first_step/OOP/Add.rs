//特征加上泛型，这里RHS的类型和Self类型相同
trait Add<RHS=Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add<u64> for u32{
    type Output = u64;
    fn add(self, other: u64) -> Self::Output{
        (self as u64) + other
    }
}

fn main(){
    let a = 1u32;
    let b = 2u64;
    assert_eq!(a.add(b), 3);
}
