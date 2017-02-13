fn main() {
    let examples = vec![
        "Hello, world!",
        "❤️ 🍑 🍀 🍍 📡 😇 🌎",
        "0⃣ 1⃣ 2⃣ 3⃣ 4⃣ 5⃣ 6⃣ 7⃣ 8⃣ 9⃣",
    ];

    for s in examples {
        let example = String::from(s);
        println!("'{}' -> '{}'", &example, reverse(&example));
    }
}

fn reverse(s: &String) -> String {
    let mut tmp = String::from(s.as_str());
    let mut result = String::new();

    loop {
        match tmp.pop() {
            Some(c) => {
                result.push(c);
            }
            None => {
                break;
            }
        }
    }

    result
}
