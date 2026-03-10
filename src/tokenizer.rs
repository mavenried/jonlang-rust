pub fn tokenize(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    let mut quote = false;

    for c in line.chars() {
        if c == '"' {
            quote = !quote;
            buf.push(c);
        } else if c == ' ' && !quote {
            if !buf.is_empty() {
                tokens.push(buf.clone());
                buf.clear();
            }
        } else {
            buf.push(c);
        }
    }

    if !buf.is_empty() {
        tokens.push(buf);
    }

    tokens
}
