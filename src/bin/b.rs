#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: String,
    }
    let len = s.len();
    let mut ans = false;

    if len == 7 && s == "keyence" {
        ans = true;
    }
    'outer: for i in 0..(len + 1) {
        for j in i + 1..(len + 1) {
            let mut str: String = s[0..i].to_string();
            str.push_str(&s[j..]);
            if str == "keyence" {
                ans = true;
                break 'outer;
            }
        }
    }
    if ans {
        println!("YES");
    } else {
        println!("NO");
    }
}
