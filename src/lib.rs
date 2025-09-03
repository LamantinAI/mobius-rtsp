pub mod config;
use crate::config::MobiusConfig;
use std::{fs, path::Path};

use gstreamer as gst;
use gstreamer_rtsp_server::prelude::*;

pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    "mp4", "avi", "mkv", "mov", "webm", "flv", "wmv", "m4v", "3gp",
];

pub fn run(config: MobiusConfig) -> Result<(), Box<dyn std::error::Error>> {
    gst::init()?;

    let server = gstreamer_rtsp_server::RTSPServer::new();
    server.set_address("0.0.0.0");
    server.set_service(&config.listen_port);

    let mounts = server.mount_points().unwrap();

    let videos_dir = "./videos";

    if !Path::new(videos_dir).exists() {
        eprintln!("Error: Directory not found {}", videos_dir);
        std::fs::create_dir_all(videos_dir)?;
        println!(
            "Directory {} created. Place your video files here.",
            videos_dir
        );
        return Ok(());
    }

    let mut video_count: u8 = 0;

    for entry in fs::read_dir(videos_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();

                if SUPPORTED_EXTENSIONS.contains(&ext.as_str()) {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();

                    // Remove the extension for the stream name
                    let stream_name =
                        if let Some(stem) = Path::new(file_name_str.as_ref()).file_stem() {
                            stem.to_string_lossy().to_string()
                        } else {
                            file_name_str.to_string()
                        };

                    // Universal pipeline: decoding + encoding in H.264
                    let stream_pipeline_str = format!(
                        "filesrc location={} ! decodebin ! videoconvert ! video/x-raw,format=I420 ! x264enc speed-preset=ultrafast tune=zerolatency ! rtph264pay name=pay0 pt=96",
                        path.to_string_lossy()
                    );

                    let factory = gstreamer_rtsp_server::RTSPMediaFactory::new();
                    factory.set_launch(&stream_pipeline_str);
                    factory.set_shared(config.shared);

                    let mount_path = format!("/{}/{}", config.prefix, stream_name);
                    mounts.add_factory(&mount_path, factory);

                    println!(
                        "Added stream: rtsp://0.0.0.0:{}{}",
                        config.listen_port, mount_path
                    );
                    video_count += 1;
                }
            }
        }
    }

    if video_count == 0 {
        println!("No video files found in directory {}", videos_dir);
        println!("Supported formats: {}", SUPPORTED_EXTENSIONS.join(", "));
    } else {
        println!("Total threads added: {}", video_count);
    }

    server.attach(None)?;

    println!("\nRTSP Server running on port {}", config.listen_port);
    println!("Press Ctrl+C to stop");

    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();
    return Ok(());
}
