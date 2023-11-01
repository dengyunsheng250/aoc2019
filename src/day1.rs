use std::cmp;
use std::fs;
pub fn run() -> usize {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut res = 0;
    for mut i in content
        .lines()
        .map(|x| x.parse::<usize>())
        .map(Result::unwrap)
    {
        while i > 0 {
            if let Some(a) = (i / 3).checked_sub(2) {
                res += a;
                i = a;
            } else {
                break;
            }
        }
    }
    res
}
