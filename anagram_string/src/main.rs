fn is_anagram(s: &str, t: &str) -> bool {
    let mut str1: Vec<char> = s.chars().collect();
    let mut str2: Vec<char> = t.chars().collect();
    str1.sort();
    str2.sort();

    if str1 == str2 {
        return true
    } else {
        return false
    }
}

fn main() {
    let anagram = is_anagram("iracema", "america");
    println!("{}", anagram);

    let anagram1 = is_anagram("cool", "col");
    println!("{}", anagram1);
}
