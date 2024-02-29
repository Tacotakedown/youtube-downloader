use crate::converter::{convert_to_mkv, convert_to_mp3, convert_to_wav};
use crate::error_wrapper::ErrorWrapper;
use pin_utils::pin_mut;
use reqwest::get;
use std::error::Error;
use std::path::{self, PathBuf};
use tauri::Window;
use tokio::io::AsyncWriteExt;
use tokio::sync::oneshot;
use tokio::{fs::rename, fs::File as TokioFile, time::Instant};
use tokio_stream::StreamExt;

use rustube::{Id, Stream, Video};

#[derive(Clone, Debug)]
enum VideoFormat {
    MP4,
    MP3,
    MKV,
    WAV,
}

#[derive(Clone, serde::Serialize)]
struct DownloadProgress {
    downloaded: f64,
    download_speed: u64,
    id: String,
}

#[derive(Clone, serde::Serialize)]
struct DownloadSize {
    size: f64,
    id: String,
}

#[derive(Clone, serde::Serialize)]
struct DownloadStatusEnum {
    status: String,
    id: String,
}

#[tauri::command]
pub async fn download_youtube_video(
    format: String,
    path: String,
    id: String,
    window: Window,
) -> Result<(), String> {
    println!("downloading video: {} to: {} as: {}", id, path, format);
    let pathbuf = PathBuf::from(path);
    let window_clone = window.clone();

    let video_format = match format.to_lowercase().as_str() {
        "mp4" => VideoFormat::MP4,
        "mp3" => VideoFormat::MP3,
        "mkv" => VideoFormat::MKV,
        "wav" => VideoFormat::WAV,
        _ => return Err("Unsupported video format".to_string()),
    };

    let video_format_clone = video_format.clone();

    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let downloaded_path =
            match download_to_in_thread(pathbuf.clone(), &id, window, window_clone, &video_format)
                .await
            {
                Some(path) => path,
                None => {
                    if let Err(_) = tx.send(Err("Failed to download video".to_string())) {
                        eprintln!("Failed to send result back to main task");
                    }
                    return;
                }
            };

        // Send the result back to the main task using the oneshot channel
        if let Err(_) = tx.send(Ok(downloaded_path)) {
            eprintln!("Failed to send result back to main task");
        }
    });

    // Await the result from the spawned task
    let downloaded_path_result = rx
        .await
        .unwrap_or_else(|_| Err("Failed to spawn download task".to_string()))?;

    match video_format_clone {
        // we need a copy of the 1st
        VideoFormat::MP4 => {
            return Ok(());
        }
        VideoFormat::MP3 => convert_to_mp3(downloaded_path_result),
        VideoFormat::MKV => convert_to_mkv(downloaded_path_result),
        VideoFormat::WAV => convert_to_wav(downloaded_path_result),
    }

    Ok(())
}

async fn download_to_in_thread(
    path: PathBuf,
    id_str: &str,
    window: Window,
    window_clone: Window,
    output: &VideoFormat,
) -> Option<PathBuf> {
    let id = Id::from_str(id_str).ok()?;
    let video = Video::from_id(id.into_owned()).await.ok()?;

    let desired_stream: &Stream;
    match output {
        VideoFormat::MP4 | VideoFormat::MKV => {
            desired_stream = video.best_video()?;
        }
        _ => {
            desired_stream = video.best_audio()?;
        }
    }

    let cipher = &desired_stream.signature_cipher;

    let url = cipher.url.as_str();

    println!("url: {}", url);

    let output_path = download_async(url, path, &id_str, window, window_clone)
        .await
        .unwrap();

    Some(output_path)
}

async fn download_async(
    url: &str,
    path: PathBuf,
    id: &str,
    _window: Window,
    window_clone: Window,
) -> Result<PathBuf, ErrorWrapper> {
    let download_path = path.clone();
    let temp_file_path = download_path.join(format!("{}.temp_download", id));
    let final_file_path = download_path.join(format!("{}.mp4", id));

    window_clone
        .emit(
            "download-state",
            DownloadStatusEnum {
                status: format!("DOWNLOADING"),
                id: id.to_string(),
            },
        )
        .unwrap();

    let mut temp_file = TokioFile::create(&temp_file_path)
        .await
        .map_err(|e| ErrorWrapper(Box::new(e) as Box<dyn Error>))?;

    let response = get(url)
        .await
        .map_err(|e| ErrorWrapper(Box::new(e) as Box<dyn Error>))?;

    if !response.status().is_success() {
        return Err(ErrorWrapper(Box::new(reqwest::Error::from(
            response.error_for_status().unwrap_err(),
        ))));
    }

    let mut downloaded_bytes = 0;
    let start_time = Instant::now();

    let content_length = response.content_length().unwrap_or(1);

    window_clone
        .emit(
            "download-size",
            DownloadSize {
                size: (content_length as f64) / (1024.0 * 1024.0),
                id: id.to_string(),
            },
        )
        .unwrap();

    let stream = response.bytes_stream();
    pin_mut!(stream);

    let mut loop_counter = 0;
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| ErrorWrapper(Box::new(e) as Box<dyn Error>))?;

        temp_file
            .write_all(&chunk)
            .await
            .map_err(|e| ErrorWrapper(Box::new(e) as Box<dyn Error>))?;

        downloaded_bytes += chunk.len() as u64;

        let elapsed_time = start_time.elapsed().as_secs_f64();
        let speed = (downloaded_bytes as f64) / (1024.0 * elapsed_time);

        loop_counter += 1;

        if loop_counter % 100 == 0 || loop_counter == 1 {
            window_clone
                .emit(
                    "download-progress",
                    DownloadProgress {
                        downloaded: (downloaded_bytes as f64) / (1024.0 * 1024.0),
                        download_speed: speed as u64,
                        id: id.to_string(),
                    },
                )
                .unwrap();
        }
    }

    rename(temp_file_path, final_file_path.clone())
        .await
        .map_err(|e| ErrorWrapper(Box::new(e) as Box<dyn Error>))?;

    window_clone
        .emit(
            "download-state",
            DownloadStatusEnum {
                status: format!("COMPLETE"),
                id: id.to_string(),
            },
        )
        .unwrap();

    Ok(final_file_path)
}
