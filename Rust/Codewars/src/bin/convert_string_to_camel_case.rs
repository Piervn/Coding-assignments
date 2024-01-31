fn main() {
    println!(
        "{}",
        to_camel_case("the-stealth-warrior") == "theStealthWarrior"
    );
    println!(
        "{}",
        to_camel_case("The_Stealth_Warrior") == "TheStealthWarrior"
    );
    println!("{}", to_camel_case("A-B-C") == "ABC");
    println!("{}", to_camel_case("a_B-C") == "aBC");
}

fn to_camel_case(text: &str) -> String {
    let mut parts = text.split(|c: char| c == '-' || c == '_');
    let first_word = parts
        .next()
        .unwrap_or("")
        .chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { c } else { c.to_ascii_lowercase() });
    let rest = parts.map(|word| {
        word.chars().enumerate().map(|(i, c)| {
            if i == 0 {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            }
        })
    });
    return first_word.chain(rest.flatten()).collect();
}
