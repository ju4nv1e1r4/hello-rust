fn pig_latin(word: &String) -> String {
    let vog = ["a", "e", "i", "o", "u"];
    let (first_letter, the_rest) = word.split_at(1);
    let first_vog = vog.contains(&first_letter);

    if first_vog{
        return format!("{}-hay", word)
    } 

    return format!("{}-{}ay", the_rest, first_letter) 
}

fn main() {
    let word = String::from("ananindeua");

    println!("{}", pig_latin(&word))
}
