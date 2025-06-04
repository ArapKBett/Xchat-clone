use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::fs;
use std::io::{self, Write};
use bytes::Bytes;

pub async fn save_file(mut payload: Multipart) -> Result<String, io::Error> {
    let mut file_path = String::new();
    while let Some(item) = payload.next().await {
        let field = item.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        let filename = field
            .content_disposition()
            .map(|cd| cd.get_filename().unwrap_or("unknown").to_string())
            .unwrap_or("unknown".to_string());
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
