use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input!{
     m:i32,
     h:i32
    }

    if h % m == 0 {
        println!("Yes")
    }
    else {
        println!("No")
    }
}
