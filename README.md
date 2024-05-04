# Windows Capture &emsp; [![Licence]][Licence URL] [![Build Status]][repository] [![Latest Version]][crates.io]

[Licence]: https://img.shields.io/crates/l/windows-capture
[Licence URL]: https://github.com/NiiightmareXD/windows-capture/blob/main/LICENCE

[Build Status]: https://img.shields.io/github/actions/workflow/status/NiiightmareXD/windows-capture/rust.yml
[repository]: https://github.com/NiiightmareXD/windows-capture

[Latest Version]: https://img.shields.io/crates/v/windows-capture
[crates.io]: https://crates.io/crates/windows-capture

**Windows Capture** is a highly efficient Rust and Python library that enables you to capture the screen using the Graphics Capture API effortlessly. This library allows you to easily capture the screen of your Windows-based computer and use it for various purposes, such as creating instructional videos, taking screenshots, or recording your gameplay. With its intuitive interface and robust functionality, Windows Capture is an excellent choice for anyone looking for a reliable, easy-to-use screen-capturing solution.

**Note** this README.md is for [Rust library](https://github.com/NiiightmareXD/windows-capture) Python library can be found [here](https://github.com/NiiightmareXD/windows-capture/tree/main/windows-capture-python)

## Features

- Only Updates The Frame When Required.
- High Performance.
- Easy To Use.
- Latest Screen Capturing API.

## Installation

Add this library to your `Cargo.toml`:

```toml
[dependencies]
windows-capture = "1.1.8"
```
or run this command

```
cargo add windows-capture
```

## Usage

```rust
use std::{
    io::{self, Write},
    time::Instant,
};

use windows_capture::{
    capture::GraphicsCaptureApiHandler,
    encoder::{VideoEncoder, VideoEncoderQuality, VideoEncoderType},
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
};

// This struct will be used to handle the capture events.
struct Capture {
    // The video encoder that will be used to encode the frames.
    encoder: Option<VideoEncoder>,
    // To measure the time the capture has been running
    start: Instant,
}

impl GraphicsCaptureApiHandler for Capture {
    // The type of flags used to get the values from the settings.
    type Flags = String;

    // The type of error that can occur during capture, the error will be returned from `CaptureControl` and `start` functions.
    type Error = Box<dyn std::error::Error + Send + Sync>;

    // Function that will be called to create the struct. The flags can be passed from settings.
    fn new(message: Self::Flags) -> Result<Self, Self::Error> {
        println!("Got The Flag: {message}");

        let encoder = VideoEncoder::new(
            VideoEncoderType::Mp4,
            VideoEncoderQuality::HD1080p,
            1920,
            1080,
            "video.mp4",
        )?;

        Ok(Self {
            encoder: Some(encoder),
            start: Instant::now(),
        })
    }

    // Called every time a new frame is available.
    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        print!(
            "\rRecording for: {} seconds",
            self.start.elapsed().as_secs()
        );
        io::stdout().flush()?;

        // Send the frame to the video encoder
        self.encoder.as_mut().unwrap().send_frame(frame)?;

        // Note: The frame has other uses too for example you can save a single for to a file like this:
        // frame.save_as_image("frame.png", ImageFormat::Png)?;
        // Or get the raw data like this so you have full control:
        // let data = frame.buffer()?;

        // Stop the capture after 6 seconds
        if self.start.elapsed().as_secs() >= 6 {
            // Finish the encoder and save the video.
            self.encoder.take().unwrap().finish()?;

            capture_control.stop();

            // Because there wasn't any new lines in previous prints
            println!();
        }

        Ok(())
    }

    // Optional handler called when the capture item (usually a window) closes.
    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture Session Closed");

        Ok(())
    }
}

fn main() {
    // Gets The Foreground Window, Checkout The Docs For Other Capture Items
    let primary_monitor = Monitor::primary().expect("There is no primary monitor");

    let settings = Settings::new(
        // Item To Captue
        primary_monitor,
        // Capture Cursor Settings
        CursorCaptureSettings::Default,
        // Draw Borders Settings
        DrawBorderSettings::Default,
        // The desired color format for the captured frame.
        ColorFormat::Rgba8,
        // Additional flags for the capture settings that will be passed to user defined `new` function.
        "Yea This Works".to_string(),
    );

    // Starts the capture and takes control of the current thread.
    // The errors from handler trait will end up here
    Capture::start(settings).expect("Screen Capture Failed");
}
```

## Documentation

Detailed documentation for each API and type can be found [here](https://docs.rs/windows-capture).

## Contributing

Contributions are welcome! If you find a bug or want to add new features to the library, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).
