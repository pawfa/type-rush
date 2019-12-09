
pub fn scan(code: &str) {
    let tokens: Vec<&str> = code.split(&[':', ' '][..]).collect();

    for x in tokens {
        match x {
            "export" => println!("The second element is test"),
            _ => println!("The second"),
        }
    }

}