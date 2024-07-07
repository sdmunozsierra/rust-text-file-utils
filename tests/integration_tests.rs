use rust_text_file_utils::file::{merge, read, write};
use rust_text_file_utils::parser::srt;
use rust_text_file_utils::text::{clean, flatten, replace, search};
use std::fs::{self, File};
use std::io::Write;

#[test]
fn test_search() {
    let result = search::find("world", "Hello, world!");
    assert_eq!(result, Some(7));
}

#[test]
fn test_replace() {
    let result = replace::replace("world", "Rust", "Hello, world!");
    assert_eq!(result, "Hello, Rust!");
}

#[test]
fn test_merge_files() {
    let file1_path = "test_file1.txt";
    let file2_path = "test_file2.txt";
    let output_path = "merged_output.txt";

    let contents1 = "Hello, ";
    let contents2 = "world!";

    // Create test files
    let mut file1 = File::create(file1_path).expect("Unable to create test file 1");
    let mut file2 = File::create(file2_path).expect("Unable to create test file 2");
    file1
        .write_all(contents1.as_bytes())
        .expect("Unable to write to test file 1");
    file2
        .write_all(contents2.as_bytes())
        .expect("Unable to write to test file 2");

    // Merge files
    merge::merge_files(output_path, &[file1_path, file2_path]).expect("Unable to merge files");

    // Verify the output
    let merged_contents =
        fs::read_to_string(output_path).expect("Unable to read merged output file");
    assert_eq!(merged_contents, "Hello, world!");

    // Clean up
    fs::remove_file(file1_path).expect("Unable to delete test file 1");
    fs::remove_file(file2_path).expect("Unable to delete test file 2");
    fs::remove_file(output_path).expect("Unable to delete merged output file");
}

#[test]
fn test_read_file_success() {
    // Create a test file with some content
    let test_path = "test_success.txt";
    let mut file = File::create(test_path).unwrap();
    writeln!(file, "This is a test file.").unwrap();

    let result = read::read_file(test_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "This is a test file.\n");

    // Clean up
    std::fs::remove_file(test_path).unwrap();
}

#[test]
fn test_read_file_not_found() {
    let result = read::read_file("non_existent_file.txt");
    assert!(result.is_err());
    let error = result.unwrap_err();
    println!("Error: {:?}", error);
}

#[test]
fn test_read_file_read_error() {
    // Assuming there's a file that exists but has read permission denied
    let result = read::read_file("/root/protected_file.txt");
    assert!(result.is_err());
    let error = result.unwrap_err();
    println!("Error: {:?}", error);
}

#[test]
fn test_read_files_sequentially() {
    // Create a test directory with some test files
    let test_dir = "test_dir";
    std::fs::create_dir(test_dir).unwrap();

    let file_names = vec!["1_first.txt", "2_second.txt", "10_third.txt"];
    let file_contents = vec!["First file.", "Second file.", "Third file."];

    for (name, content) in file_names.iter().zip(&file_contents) {
        let mut file = File::create(format!("{}/{}", test_dir, name)).unwrap();
        writeln!(file, "{}", content).unwrap();
    }

    let result = read::read_files_sequentially(test_dir, "txt");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(
        result,
        vec![
            ("1_first.txt".to_string(), "First file.\n".to_string()),
            ("2_second.txt".to_string(), "Second file.\n".to_string()),
            ("10_third.txt".to_string(), "Third file.\n".to_string())
        ]
    );

    // Clean up
    for name in file_names {
        std::fs::remove_file(format!("{}/{}", test_dir, name)).unwrap();
    }
    std::fs::remove_dir(test_dir).unwrap();
}

#[test]
fn test_write_file() {
    let file_path = "test_write.txt";
    let contents = "Hello, world!";

    // Write to file
    let result = write::write_file(file_path, contents);
    assert!(
        result.is_ok(),
        "Failed to write to file: {:?}",
        result.err()
    );

    // Verify the contents
    let written_contents = fs::read_to_string(file_path).expect("Unable to read test file");
    assert_eq!(written_contents, contents);

    // Clean up
    fs::remove_file(file_path).expect("Unable to delete test file");
}

#[test]
fn test_write_file_fail() {
    let invalid_file_path = "/invalid_path/test_write.txt";
    let contents = "Hello, world!";

    // Attempt to write to an invalid path
    let result = write::write_file(invalid_file_path, contents);
    assert!(result.is_err(), "Expected an error but got Ok");

    // Verify the error message
    if let Err(e) = result {
        assert!(
            e.to_string().contains("Failed to create file"),
            "Unexpected error message: {}",
            e
        );
    }
}

#[test]
fn test_flatten_text() {
    let text = "This is a line.\nThis is another line.\nAnd another one.";
    let expected = "This is a line.\nThis is another line.\nAnd another one.";
    assert_eq!(flatten::flatten_text(text), expected);

    let text_with_no_period = "This is a line without period\nstill no period\nfinally a period.";
    let expected_with_no_period = "This is a line without period still no period finally a period.";
    assert_eq!(
        flatten::flatten_text(text_with_no_period),
        expected_with_no_period
    );

    let text_with_commas = "First part, \nsecond part,\nthird part.";
    let expected_with_commas = "First part, second part, third part.";
    assert_eq!(
        flatten::flatten_text(text_with_commas),
        expected_with_commas
    );

    let text_with_exclamations = "Wow!\nThis is amazing!\nIncredible!";
    let expected_with_exclamations = "Wow!\nThis is amazing!\nIncredible!";
    assert_eq!(
        flatten::flatten_text(text_with_exclamations),
        expected_with_exclamations
    );

    let text_with_questions = "Is this a test?\nYes, it is.\nAre we sure?";
    let expected_with_questions = "Is this a test?\nYes, it is.\nAre we sure?";
    assert_eq!(
        flatten::flatten_text(text_with_questions),
        expected_with_questions
    );

    let mixed_punctuation = "Start with a statement.\nThen a question?\nAn exclamation!";
    let expected_mixed_punctuation = "Start with a statement.\nThen a question?\nAn exclamation!";
    assert_eq!(
        flatten::flatten_text(mixed_punctuation),
        expected_mixed_punctuation
    );

    let complex_text = "First part,\nsecond part.\nThird part?\nYes, indeed!";
    let expected_complex_text = "First part, second part.\nThird part?\nYes, indeed!";
    assert_eq!(flatten::flatten_text(complex_text), expected_complex_text);
}

#[test]
fn test_parse_srt_full() {
    let srt_content = r#"1

00:00:00,000  -->  00:00:02,360
Now let's actually test our prompts.

2

00:00:02,360  -->  00:00:07,500
This is just the querying the model with no system prompts.
"#;

    let expected = vec![
        srt::Subtitle {
            sequence: 1,
            start: "00:00:00,000".to_string(),
            end: "00:00:02,360".to_string(),
            text: "Now let's actually test our prompts.".to_string(),
        },
        srt::Subtitle {
            sequence: 2,
            start: "00:00:02,360".to_string(),
            end: "00:00:07,500".to_string(),
            text: "This is just the querying the model with no system prompts.".to_string(),
        },
    ];

    let result = srt::parse_srt(srt_content, srt::OutputFormat::Full).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_parse_srt_text_only() {
    let srt_content = r#"1

00:00:00,000  -->  00:00:02,360
Now let's actually test our prompts.

2

00:00:02,360  -->  00:00:07,500
This is just the querying the model with no system prompts.
"#;

    let expected = vec![
        srt::Subtitle {
            sequence: 1,
            start: String::new(),
            end: String::new(),
            text: "Now let's actually test our prompts.".to_string(),
        },
        srt::Subtitle {
            sequence: 2,
            start: String::new(),
            end: String::new(),
            text: "This is just the querying the model with no system prompts.".to_string(),
        },
    ];

    let result = srt::parse_srt(srt_content, srt::OutputFormat::TextOnly).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_invalid_sequence() {
    let srt_content = r#"A

00:00:00,000  -->  00:00:02,360
Now let's actually test our prompts.
"#;

    let result = srt::parse_srt(srt_content, srt::OutputFormat::Full);
    assert!(result.is_err());
}

#[test]
fn test_invalid_timestamp() {
    let srt_content = r#"1

00:00:00  -->  00:00:02,360
Now let's actually test our prompts.
"#;

    let result = srt::parse_srt(srt_content, srt::OutputFormat::Full);
    assert!(result.is_err());
}

#[test]
fn test_clean_title() {
    let inputs = vec![
            "17 - CD13317 GenAI C2 L1 A13 Understanding LLM Capabilities And Using Your Intuition V3 - lang_en-us.srt",
            "18 - CD13317 GenAI C2 L1 A14 Lesson Review V1 - lang_en-us.srt",
            "1 - CD13317 GenAI C2 L1 A01 Meet Your Instructor V1 - lang_en-us.srt",
            "2 - CD13317 GenAI C2 L1 A02 Lesson Overview V2 - lang_en-us.srt",
            "3 - CD13317 GenAI C2 L1 A03 Historical Recap V1 - lang_en-us.srt",
            "4 - CD13317 GenAI C2 L1 A04 Encoder Vs Decoder Models V1 - lang_en-us.srt",
            "5 - CD13317 GenAI C2 L1 A05 Completion Vs. Instruction Tuning V1 - lan.srt",
        ];

    let expected_outputs = vec![
        "Understanding LLM Capabilities And Using Your Intuition",
        "Lesson Review",
        "Meet Your Instructor",
        "Lesson Overview",
        "Historical Recap",
        "Encoder Vs Decoder Models",
        "Completion Vs. Instruction Tuning",
    ];

    for (input, expected) in inputs.iter().zip(expected_outputs.iter()) {
        assert_eq!(clean::clean_title(input), *expected);
    }
}
