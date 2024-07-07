pub fn flatten_text(text: &str) -> String {
    let mut flattened = String::new();
    let mut buffer = String::new();
    let mut last_char_was_comma = false;
    let mut needs_space = false;

    for line in text.lines() {
        let trimmed_line = line.trim();
        if !trimmed_line.is_empty() {
            if !buffer.is_empty() && !last_char_was_comma {
                needs_space = true;
            }
            for ch in trimmed_line.chars() {
                if ch == ',' {
                    buffer.push(ch);
                    last_char_was_comma = true;
                    needs_space = false;
                } else {
                    if needs_space {
                        buffer.push(' ');
                        needs_space = false;
                    }
                    buffer.push(ch);
                    last_char_was_comma = false;
                }
            }

            if trimmed_line.ends_with('.')
                || trimmed_line.ends_with('!')
                || trimmed_line.ends_with('?')
            {
                if !flattened.is_empty() {
                    flattened.push('\n');
                }
                flattened.push_str(&buffer);
                buffer.clear();
            } else {
                needs_space = true;
            }
        }
    }

    // Append any remaining buffer content if there's no ending punctuation
    if !buffer.is_empty() {
        if !flattened.is_empty() {
            flattened.push('\n');
        }
        flattened.push_str(&buffer);
    }

    flattened
}
