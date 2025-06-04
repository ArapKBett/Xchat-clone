use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::fs;
use std::io::Write;
use bytes::Bytes;

pub async fn save_file(mut payload: Multipart) -> Result<String, std::io::Error> {
    let mut file_path = String::new();
    while let Some(item) = payload.next().await.transpose()? {
        let mut field = item;
        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .and_then(|cd| cd.get_filename())
            .unwrap_or("unknown");
        let filepath = format!("/data/uploads/{}", filename);
        let mut f = fs::File::create(&filepath)?;
        while let Some(chunk) = field.next().await {
            let data: Bytes = chunk?;
            f.write_all(&data)?;
        }
        file_path = filepath;
    }
    Ok(file_path)
}
