use std::fs::File;
use std::io::{Error, Write};

use actix_multipart::Multipart;
use actix_web::error::BlockingError;
use actix_web::web;
use futures::{AsyncWriteExt, StreamExt, TryStreamExt};

pub enum UploadError {
    BeginFileCreationError(BlockingError),
    FileCreationError,
    FileWritingError
}

pub async fn save_file(mut payload: Multipart, file_path: String) -> Result<(), UploadError> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        //let filename = content_type.get_filename().unwrap();
        let filepath = format!(".{}", file_path);

        // File::create is blocking operation, use threadpool
        let mut f : File = match web::block(|| std::fs::File::create(filepath)).await {
            Ok(file_result) => match file_result {
                Ok(file) => file,
                Err(e) => return Err(UploadError::FileCreationError)
            },
            Err(e) => return Err(UploadError::BeginFileCreationError(e))
        };

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = match web::block(move || f.write_all(&data).map(|_| f)).await {
                Ok(f) => match f {
                    Ok(f) => f,
                    Err(e) => return Err(UploadError::FileWritingError)
                },
                Err(e) => return Err(UploadError::FileWritingError)
            };
        }
    }

    Ok(())
}