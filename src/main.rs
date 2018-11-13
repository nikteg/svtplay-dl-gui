#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate sciter;
extern crate reqwest;
extern crate scraper;
extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate regex;
extern crate serde_json;

use sciter::Value;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

pub mod services;

struct EventHandler {}

fn play_success_sound() {
    thread::spawn(move || {
        let mut cmd = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .args(&["-c", "(New-Object Media.SoundPlayer beep.wav).PlaySync()"])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("afplay beep.wav")
                .spawn()
                .unwrap()
        };

        cmd.wait().unwrap();
    });
}

impl EventHandler {
    fn fetch_stream_info(&self, url: String, error: sciter::Value, done: sciter::Value) -> bool {
        thread::spawn(move || {
            let stream = services::get_stream(&url);

            match stream {
                Ok(s) => {
                    // println!("Stream is: {:?}", s);
                    done.call(None, &make_args!(s.filename, s.url), None)
                        .unwrap();
                }
                Err(e) => {
                    error.call(None, &make_args!(e), None).unwrap();
                }
            }
        });

        true
    }

    fn download_stream(
        &self,
        url: String,
        filename: String,
        error: sciter::Value,
        data: sciter::Value,
        done: sciter::Value,
    ) -> bool {
        thread::spawn(move || {
            let mut cmd = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(&["/C", "echo", &url])
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap()
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(format!(
                        "ffmpeg -y -i \"{}\" -acodec copy -vcodec copy -absf aac_adtstoasc \"{}\"",
                        url, filename
                    )).stderr(Stdio::piped())
                    .spawn()
                    .unwrap()
            };

            {
                let stderr = cmd.stderr.as_mut().unwrap();
                let stderr_reader = BufReader::new(stderr);
                let stderr_lines = stderr_reader.lines();

                for line in stderr_lines {
                    // println!("Read: {:?}", line);
                    data.call(None, &make_args!(line.unwrap()), None).unwrap();
                }
            }

            cmd.wait().unwrap();
            done.call(None, &make_args!(), None).unwrap();
        });

        true
    }

    fn play_sound(&self) -> bool {
        play_success_sound();

        true
    }
}

impl sciter::EventHandler for EventHandler {
    dispatch_script_call! {
        fn download_stream(String, String, Value, Value, Value);
        fn fetch_stream_info(String, Value, Value);
        fn play_sound();
    }
}

fn main() {
    let handler = EventHandler {};
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(0x08)).ok();
    let mut frame = sciter::WindowBuilder::main_window()
        .with_size((512, 512))
        .create();
    frame.event_handler(handler);
    frame.load_html(include_bytes!("../html/main.html"), Some("html://main.htm"));
    frame.run_app();
}
