# mobius-rtsp

mobius-rtsp is a GStreamer-based RTSP server for infinite video files streaming.

**How It Works**

1. Place supported video files in the `data/videos` folder
2. On startup, the server reads video filenames (without extensions) and adds them as RTSP streams

**Example:**

videofile: data/videos/9c0a140d548c8313e7719b7590d029dc.mp4

rtsp stream: rtsp://0.0.0.0:8554/mobius-stream/9c0a140d548c8313e7719b7590d029dc

**Supported video formats:**
   - mp4
   - avi
   - mkv
   - mov
   - webm
   - flv
   - wmv
   - m4v
   - 3gp

## Prerequisites

### System Dependencies

The following GStreamer libraries and plugins are required:
- libgstreamer1.0-0
- libgstreamer-plugins-base1.0-0
- libgstreamer-plugins-good1.0-0
- libgstreamer-plugins-bad1.0-0
- libgstrtspserver-1.0-0
- gstreamer1.0-plugins-base
- gstreamer1.0-plugins-good
- gstreamer1.0-plugins-bad
- gstreamer1.0-plugins-ugly
- gstreamer1.0-x
- gstreamer1.0-tools

### Rust Compiler

Rust compiler version 1.88 or higher is required.

## Installation and Running

### Local Development

Build and run the project:
```bash
cargo run
```

### Docker

Run using Docker Compose:
```bash
docker compose up
```

## Configuration

The server can be configured using environment variables:

### Core Settings

| Variable | Default | Description |
|----------|---------|-------------|
| `MOBIUS_PORT` | 8554 | RTSP server port |
| `MOBIUS_SHARED` | true | Individual stream per client (false) or shared stream (true) |
| `MOBIUS_PREFIX` | mobius-stream | RTSP server resources prefix |
| `MOBIUS_INFINITE` | true | Streaming without EOS message |

### Network Corruption

| Variable | Default | Description |
| --- | --- | --- |
| `MOBIUS_CORRUPTED` | `false` | Enable/Disable the dynamic network sabotage thread |
| `MOBIUS_MIN_DROP_PROBABILITY` | `0.05` | Packet loss probability during "normal" operation (0.0 to 1.0) |
| `MOBIUS_MAX_DROP_PROBABILITY` | `0.9` | Packet loss probability during "outage" (0.9 is recommended for glitches) |
| `MOBIUS_MIN_TIME_BETWEEN_BREAK` | `10` | Minimum uptime (seconds) before an outage occurs |
| `MOBIUS_MAX_TIME_BETWEEN_BREAK` | `30` | Maximum uptime (seconds) before an outage occurs |
| `MOBIUS_MIN_DISCONNECTED_TIME` | `10` | Minimum duration (seconds) of the network outage |
| `MOBIUS_MAX_DISCONNECTED_TIME` | `30` | Maximum duration (seconds) of the network outage |

### Configuration Examples

**Custom port:**
```bash
MOBIUS_PORT=9000 cargo run
```

**Individual streams:**
```bash
MOBIUS_SHARED=false cargo run
```

**Resources prefix:**
```bash
MOBIUS_PREFIX=custom-stream cargo run
```

**Infinite streaming mode:**
```bash
MOBIUS_INFINITE=true cargo run
```

**Docker with custom configuration:**
```bash
docker run -p 9000:9000 -e MOBIUS_PORT=9000 -e MOBIUS_SHARED=false -v ./videos:/mobius/videos mobius-rtsp
```

## Features

- **Multi-format support**: Works with all popular video formats
- **Automatic stream discovery**: Automatically detects video files in the videos directory
- **Configurable streaming mode**: Shared or individual streams per client
- **Docker support**: Ready-to-use Docker configuration
- **Low latency**: Optimized H.264 encoding with ultrafast presets
- **Cross-platform**: Works on Linux, macOS, and Windows
