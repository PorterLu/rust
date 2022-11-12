fn main(){
    let x = 32;
    match x{
        0 => {
            println!("x is 0");
        }

        1|2 => {
            println!("1 or 2 match");
        }

        3..=9 => {
            println!("3~9 match");
        }

        match_num @ 10..=1000 => {
            println!("{}", match_num);
        }

        _ => {
            println!("something else");
        }
    }
}
