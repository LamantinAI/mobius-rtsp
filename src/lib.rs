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

    let videos_dir = "./data/videos";
    let segments_dir = Path::new("./data/segments");

    if !Path::new(videos_dir).exists() {
        eprintln!("Error: Directory not found {}", videos_dir);
        std::fs::create_dir_all(videos_dir)?;
        println!(
            "Directory {} created. Place your video files here.",
            videos_dir
        );
        return Ok(());
    }

    if config.infinite {
        if !segments_dir.exists() {
            std::fs::create_dir_all(segments_dir)?;
        }

        for entry in fs::read_dir(videos_dir)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if !SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                    continue;
                }
            } else {
                continue;
            }

            let stem = path.file_stem().unwrap().to_string_lossy().to_string();
            let output_segment_dit = segments_dir.join(&stem);

            slice_video_to_segments(&path, &output_segment_dit)?;
        }
    }

    let server = gstreamer_rtsp_server::RTSPServer::new();
    server.set_address("0.0.0.0");
    server.set_service(&config.port.to_string());

    let mounts = server.mount_points().unwrap();

    let mut video_count: u8 = 0;

    for entry in fs::read_dir(videos_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if !SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                continue;
            }
        } else {
            continue;
        }

        let stem = path.file_stem().unwrap().to_string_lossy().to_string();
        let mount_path = format!("/{}/{}", config.prefix, stem);

        // Сonfiguring the factory string
        let stream_pipeline_str = if config.infinite {
            let segment_pattern = segments_dir.join(&stem).join("segment%03d.ts");
            format!(
                "multifilesrc location={} loop=true ! tsparse ! tsdemux ! h264parse ! rtph264pay name=pay0 pt=96",
                segment_pattern.to_string_lossy()
            )
        } else {
            format!(
                "filesrc location={} ! decodebin ! videoconvert ! video/x-raw,format=I420 ! openh264enc complexity=high multi-thread=4 ! rtph264pay name=pay0 pt=96",
                path.to_string_lossy()
            )
        };

        let factory = gstreamer_rtsp_server::RTSPMediaFactory::new();
        factory.set_launch(&stream_pipeline_str);
        factory.set_shared(config.shared);

        mounts.add_factory(&mount_path, factory);
        println!("Added stream: rtsp://0.0.0.0:{}{}", config.port, mount_path);
        video_count += 1;
    }

    if video_count == 0 {
        println!("No video files found in directory {}", videos_dir);
        println!("Supported formats: {}", SUPPORTED_EXTENSIONS.join(", "));
    } else {
        println!("Total threads added: {}", video_count);
    }

    server.attach(None)?;

    println!("\nRTSP Server running on port {}", config.port);
    println!("Press Ctrl+C to stop");

    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();
    return Ok(());
}

fn slice_video_to_segments(
    video_path: &Path,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check our video segments dir
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)?;
    }

    // Check if there are any segments
    if output_dir.read_dir()?.next().is_some() {
        println!("Segments already exist in {:?}", output_dir);
        return Ok(());
    }

    println!("Slicing {:?} into segments in {:?}", video_path, output_dir);

    // Сonfiguring the slicer string
    let location_pattern = output_dir.join("segment%03d.ts");
    let pipeline_str = format!(
        "filesrc location={} ! decodebin ! videoconvert ! videoscale ! \
        video/x-raw,format=I420,width=1280,height=720,framerate=30/1 ! \
        openh264enc \
            bitrate=6000000 \
            gop-size=30 \
            usage-type=camera \
            rate-control=buffer \
            complexity=high \
            multi-thread=4 \
        ! h264parse ! \
        splitmuxsink muxer=\"mpegtsmux alignment=7\" max-size-time=1000000000 location={}",
        video_path.to_string_lossy(),
        location_pattern.to_string_lossy()
    );

    // Creating slicer pipeline
    let pipeline = gst::parse::launch(&pipeline_str)
        .map_err(|e| format!("Failed to parse pipeline: {}", e))?;

    let pipeline = pipeline
        .dynamic_cast::<gst::Pipeline>()
        .map_err(|_| "Parsed element is not a Pipeline")?;

    // Getting bus of the pipeline
    let bus = pipeline.bus().ok_or("Pipeline has no bus")?;

    // Launching the pipeline
    pipeline
        .set_state(gst::State::Playing)
        .map_err(|e| format!("Failed to set pipeline to Playing: {:?}", e))?;

    // Waiting until EOS or Error
    while let Some(msg) = bus.timed_pop(gst::ClockTime::NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Eos(..) => {
                println!("Reached end of stream for {:?}", video_path);
                break;
            }
            MessageView::Error(err) => {
                eprintln!(
                    "Error in slicing pipeline: {} (debug: {:?})",
                    err.error(),
                    err.debug()
                );
                pipeline.set_state(gst::State::Null)?;
                return Err("Error during video slicing".into());
            }
            _ => continue,
        }
    }

    // Stopping the pipeline correctly
    pipeline.set_state(gst::State::Null)?;

    println!("Slicing completed for {:?}", video_path);
    Ok(())
}
