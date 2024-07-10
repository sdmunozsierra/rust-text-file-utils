use rust_text_file_utils::file::unzip;

#[tokio::main]
async fn main() {
    match unzip::unzip_file(
        "/home/adminlenovo/Data/Downloads/NLP_Fundamentals Subtitles.zip",
        "/home/adminlenovo/Data/Learning/Udacity/Introduction_to_LLMs",
    )
    .await
    {
        Ok(_) => println!("Unzip successful!"),
        Err(e) => eprintln!("Error unzipping file: {}", e),
    }
}
