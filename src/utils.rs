use actix_multipart::Multipart;
use futures_util::stream::StreamExt;
use std::fs;
use std::io::Write;

pub async fn save_file(mut payload: Multipart, path: &str) -> Result<String, std::io::Error> {
    let mut file_path = String::new();
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition().unwrap();
        let filename = content_disposition.get_filename().unwrap_or("unknown");
        let filepath = format!("{}/{}", path, filename);
        let mut f = fs::File::create(&filepath)?;
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data)?;
        }
        file_path = filepath;
    }
    Ok(file_path)
}
