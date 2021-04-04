use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input!{
        a:i32,
        b:i32,
        w_kg:i32
    }

    let w = w_kg * 1000;

    let mut min = -1;
    let mut max = -1;


    let x_max = w / a + 1;



    if max == -1 || min == -1{
        println!("UNSATISFIABLE")
    }
    else {
        println!("{} {}",min, max)
    }
}
