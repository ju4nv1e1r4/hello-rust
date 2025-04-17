fn one_step_away(str1: &str, str2: &str) -> bool {
    let len1 = str1.len();
    let len2 = str2.len();

    if (len1 as i32 - len2 as i32).abs() > 1 {
        return false;
    }

    let mut edits = 0;
    let mut len_str1 = 0;
    let mut len_str2 = 0;

    while len_str1 < len1 && len_str2 < len2 {
        if str1.chars().nth(len_str1) != str2.chars().nth(len_str2) {
            edits += 1;

            if len1 > len2 {
                len_str1 += 1;
            } else if len1 < len2 {
                len_str2 += 1;
            } else {
                len_str1 += 1;
                len_str2 += 1;
            }
        } else {
            len_str1 += 1;
            len_str2 += 1;
        }

        if edits > 1 {
            return false;
        }
    }

    if len_str1 < len1 || len_str2 < len2 {
        edits += 1
    }

    edits <= 1
}

fn main() {
    println!("=> Same lenght of chars...");
    let mut string1 = "juan";
    let mut string2 = "caio";

    let distance = one_step_away(string1, string2);
    println!("Comparação entre {} e {}", string1, string2);
    println!("Resultado: {}", distance);

    println!("\n=> Removing 'a' char...");
    string1 = "pale";
    string2 = "ple";

    let distance = one_step_away(string1, string2);
    println!("Comparação entre {} e {}", string1, string2);
    println!("Resultado: {}", distance);
}
