fn main() {
    let s = "abcabcbb";
    println!("{}", length_of_longest_substring(s.to_string()));
}

fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;
    let mut start = 0;
    let mut end = 0;
    let mut map = std::collections::HashMap::new();
    for c in s.chars() {
        if let Some(&i) = map.get(&c) {
            start = std::cmp::max(start, i + 1);
        }
        end += 1;
        map.insert(c, end - 1);
        max = std::cmp::max(max, end - start);
    }
    max as i32
}