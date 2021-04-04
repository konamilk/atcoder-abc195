use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input!{
    n: i64
    }

    let mut ans = 0i64;
    if n / 1_000 == 0{
        ans = 0i64;
    }
    else if n / 1_000_000 == 0{
        let x = 1_000;
        ans = 1 * (n % x + ((n - x)/x) * x + 1)
    }
    else if n / 1_000_000_000 == 0{
        let x = 1_000_000;
        ans = 2 * (n % x + ((n - x)/x) * x + 1) + 1_000
    }
    else if n / 1_000_000_000_000 == 0{
        let x = 1_000_000_000;
        ans = 3 * (n % x + ((n - x)/x) * x + 1) + 2_000_000
    }
    else {
        let x = 1_000_000_000;
        ans = 4 * (n % x + ((n - x)/x) * x + 1) + 3_000_000_0000

    }
    println!("{}", ans)
}
