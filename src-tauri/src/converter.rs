use std::{
    path::PathBuf,
    process::{exit, Command},
};

fn convert(input_file: &str, output_file: &str, format: &str) {
    let status = Command::new("ffmpeg")
        .args(&[
            "-i",
            input_file,
            "-acodec",
            match format {
                "mp3" => "libmp3lame",
                "wav" => "pcm_s16le",
                "mkv" => "copy",
                _ => {
                    eprintln!("Unsupported output format");
                    exit(1);
                }
            },
            output_file,
        ])
        .status()
        .expect("Failed to execute FFmpeg");

    if status.success() {
        println!("Conversion to {} successful!", format);
    } else {
        eprintln!("Conversion to {} failed!", format);
    }
}

pub fn convert_to_mp3(path: PathBuf) {
    let input_file = path.to_string_lossy();
    let output_path = path.with_extension("mp3");
    let output_file = output_path.to_string_lossy();
    convert(&input_file, &output_file, "mp3");
}

pub fn convert_to_mkv(path: PathBuf) {
    let input_file = path.to_string_lossy();
    let output_path = path.with_extension("mkv");
    let output_file = output_path.to_string_lossy();
    convert(&input_file, &output_file, "mkv");
}

pub fn convert_to_wav(path: PathBuf) {
    let input_file = path.to_string_lossy();
    let output_path = path.with_extension("wav");
    let output_file = output_path.to_string_lossy();
    convert(&input_file, &output_file, "wav");
}
