fn main() {
    let text = "fn main() {
    let text = \"?\";
    for ch in text.chars() {
        if ch == 63 as char {
            for ch in text.chars() {
                if ch == '\"' {
                    print!(\"{}{}\", 92 as char, 34 as char);
                } else {
                    print!(\"{}\", ch);
                }
            }
        } else {
            print!(\"{}\", ch);
        }
    }
}
";
    for ch in text.chars() {
        if ch == 63 as char {
            for ch in text.chars() {
                if ch == '"' {
                    print!("{}{}", 92 as char, 34 as char);
                } else {
                    print!("{}", ch);
                }
            }
        } else {
            print!("{}", ch);
        }
    }
}
