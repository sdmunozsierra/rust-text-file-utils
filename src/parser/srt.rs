use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Subtitle {
    pub sequence: usize,
    pub start: String,
    pub end: String,
    pub text: String,
}

pub enum OutputFormat {
    Full,
    TextOnly,
}

pub fn parse_srt(srt_content: &str, format: OutputFormat) -> Result<Vec<Subtitle>, String> {
    let sequence_re = Regex::new(r"^\d+$").unwrap();
    let timestamp_re =
        Regex::new(r"^\d{2}:\d{2}:\d{2},\d{3}\s*-->\s*\d{2}:\d{2}:\d{2},\d{3}$").unwrap();

    let mut subtitles = Vec::new();
    let mut lines = srt_content.lines().peekable();

    while let Some(line) = lines.next() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Parse sequence number
        let sequence = if sequence_re.is_match(line) {
            line.parse::<usize>()
                .map_err(|e| format!("Invalid sequence number: {}", e))?
        } else {
            return Err(format!("Invalid sequence format: {}", line));
        };

        // Skip the empty line before the timestamp
        lines
            .next()
            .ok_or("Expected a blank line before the timestamp but found none")?;

        // Parse timestamp
        let timestamp_line = lines.next().ok_or("Missing timestamp line")?;
        if !timestamp_re.is_match(timestamp_line.trim()) {
            return Err(format!("Invalid timestamp format: {}", timestamp_line));
        }
        let timestamps: Vec<&str> = timestamp_line.split("-->").collect();
        let start = timestamps[0].trim().to_string();
        let end = timestamps[1].trim().to_string();

        // Parse subtitle text
        let mut text = String::new();
        while let Some(text_line) = lines.peek() {
            if text_line.trim().is_empty() {
                break;
            }
            text.push_str(lines.next().unwrap());
            text.push('\n');
        }
        text = text.trim_end().to_string(); // Remove trailing newline

        let subtitle = match format {
            OutputFormat::Full => Subtitle {
                sequence,
                start,
                end,
                text,
            },
            OutputFormat::TextOnly => Subtitle {
                sequence,
                start: String::new(),
                end: String::new(),
                text,
            },
        };

        subtitles.push(subtitle);
    }

    Ok(subtitles)
}
