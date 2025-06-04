use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::fs;
use std::io::{self, Write};
use bytes::Bytes;
use actix_web::http::header::ContentDisposition;

pub async fn save_file(mut payload: Multipart) -> Result<String, io::Error> {
    let mut file_path = String::new();
    while let Some(item) = payload.next().await {
        let field = item.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        
        // Correct way to extract the filename
        let filename = field.content_disposition()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing content disposition"))?
            .get_filename()
            .unwrap_or("unknown")
            .to_string();
        
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
