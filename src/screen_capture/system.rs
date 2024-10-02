use std::os::raw::c_void;
use std::path::Path;
use std::process::Command;

use gl;

use image;
/// my attempt at a screen capture system  
/// obs causes flactuations when capturing my window making my fps go bananas  
/// couldn't really figure aout the problems to im just going to make a screen capture system  
/// i'm sure its not that difficult...
#[derive(Debug, Default)]
pub struct ScreenCapture {
    stream: Vec<image::DynamicImage>,

    width: u32,
    height: u32,
}

impl ScreenCapture {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            stream: Vec::new(),
        }
    }

    pub fn update_dimensions(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn get_frame(&self) -> Result<image::DynamicImage, String> {
        let mut frame = image::DynamicImage::new_rgba8(self.width, self.height);

        unsafe {
            let pixels = frame.as_mut_rgba8().unwrap();
            let pixels_buffer_ptr = pixels.as_mut_ptr() as *mut c_void;
            // always cupture the full frame
            // might change this later but for now always capture from the top left
            let x = 0;
            let y = 0;

            gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
            gl::ReadPixels(
                x,
                y,
                self.width as i32,
                self.height as i32,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels_buffer_ptr,
            );
            let error_code = gl::GetError();
            if error_code != gl::NO_ERROR {
                return Err(String::from("error capturing frame!"));
            }
        }
        Ok(frame)
    }

    ///capture frames to be converted into a video later
    pub fn capture(&mut self) {
        self.stream.push(self.get_frame().unwrap());
    }
    /// yeah... saving as a png first before stitching them into the final video is not the fastest solution  
    /// infact, its incredibly slow might look for a faster solution in the future but for now if it works it works
    pub fn save_video(&self, destination: &Path) {
        let tmp_dir = std::env::temp_dir();

        let total_frames = self.stream.len();
        self.stream.iter().enumerate().for_each({
            |(i, capture)| {
                let img_path = tmp_dir.join(format!("frame_{:04}.png", i));

                eprint!("\rprocessing frame {i} out of {total_frames}");

                capture
                    .flipv()
                    .save_with_format(img_path, image::ImageFormat::Png)
                    .unwrap();
            }
        });
        eprintln!("\ndone processing frames");

        let status = Command::new("ffmpeg")
            .args([
                "-framerate",
                "60",
                "-i",
                &tmp_dir.join("frame_%04d.png").to_str().unwrap(),
                "-c:v",
                "libx264",
                "-r",
                "60",
                destination.to_str().unwrap(),
            ])
            .status()
            .unwrap();

        if !status.success() {
            println!("failed to capture screen! ");
        }
    }

    /// capture a particular frame and save it with the specified destination name
    /// format determined by name
    pub fn screen_shot(&self, destination: &Path) {
        let frame = self.get_frame().unwrap();
        frame.save(destination).unwrap();
    }
}
