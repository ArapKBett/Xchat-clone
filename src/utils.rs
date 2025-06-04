use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::fs;
use std::io::{self, Write};
use bytes::Bytes;

pub async fn save_file(mut payload: Multipart) -> Result<String, io::Error> {
    let mut file_path = String::new();
    while let Some(item) = payload.next().await {
        let field = item.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        let content_disposition = field.content();
        let filename = content_disposition
            .get_filename()
            .map_or_else(|| "unknown".to_string(), |name| name.to_string());
        let filepath = format!("/data/uploads/{}", filename);
        let mut f = fs::File::create(&filepath)?;
        let mut field = field;
        while let Some(chunk) = field.next().await {
            let data: Bytes = chunk.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            f.write_all(&data)?;
        }
        file_path = filepath;
    }
    Ok(file_path)
}
