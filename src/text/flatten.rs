pub fn flatten_text(text: &str) -> String {
    let mut flattened = String::new();
    let mut buffer = String::new();

    for line in text.lines() {
        let trimmed_line = line.trim();
        if !trimmed_line.is_empty() {
            // If buffer is not empty, add a space before appending new trimmed line
            if !buffer.is_empty() {
                buffer.push(' ');
            }
            buffer.push_str(trimmed_line);

            // Check for sentence-ending punctuation
            if trimmed_line.ends_with('.') || trimmed_line.ends_with('!') || trimmed_line.ends_with('?') {
                if !flattened.is_empty() {
                    flattened.push('\n');
                }
                flattened.push_str(&buffer);
                buffer.clear();
            } else if trimmed_line.ends_with(',') {
                buffer.push(' ');
            }
        }
    }

    // Append any remaining buffer content if there's no ending period
    if !buffer.is_empty() {
        if !flattened.is_empty() {
            flattened.push('\n');
        }
        flattened.push_str(&buffer);
    }

    flattened
}

