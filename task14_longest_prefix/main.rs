fn main() {
    let start: Vec<String> = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let end_result = longest_common_prefix(start.to_vec());
    println!("{}", end_result);
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result = String::new();
    let mut i = 0;
    let mut j;
    let mut c;

    // find a length of the shortest string
    let mut min = strs[0].len();
    for s in strs.iter() {
        if s.len() < min {
            min = s.len();
        }
    }
    println!("min: {}", min);

    // compare chars in a first string with chars in other strings
    while i < min {
        c = strs[0].chars().nth(i).unwrap();
        j = 1;
        while j < strs.len() {
            if strs[j].chars().nth(i).unwrap() != c {
                return result;
            }
            j += 1;
        }
        result.push(c);
        i += 1;
    }
    result
}