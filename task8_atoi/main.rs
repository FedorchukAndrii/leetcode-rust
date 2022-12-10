fn main() {
    let s = "42";
    println!("{}", my_atoi(s.to_string()));
}

my_atoi(s: String) -> i32 {
    let mut result = 0;
    let mut sign = 1;
    let mut i = 0;
    let mut s = s.trim().to_string();
    if s.len() == 0 {
        return 0;
    }
    if s.chars().nth(0).unwrap() == '-' {
        sign = -1;
        s = s[1..].to_string();
    } else if s.chars().nth(0).unwrap() == '+' {
        s = s[1..].to_string();
    }
    while i < s.len() {
        let c = s.chars().nth(i).unwrap();
        if c < '0' || c > '9' {
            break;
        }
        result = result * 10 + (c as i32 - '0' as i32);
        i += 1;
    }
    result * sign
}