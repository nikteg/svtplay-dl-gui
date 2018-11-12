#[macro_use]
extern crate sciter;
extern crate reqwest;
extern crate scraper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// use reqwest;
use sciter::Value;
use scraper::Html;
use scraper::Selector;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

#[derive(Deserialize, Debug)]
struct VideoReference {
    url: String,
    format: String,
}

#[derive(Deserialize, Debug)]
struct SVTVideo {
    programTitle: String,
    episodeTitle: String,
    videoReferences: Vec<VideoReference>,
}

fn visit_website(url: &str) {
    let request = reqwest::get(url);
    let text = request.unwrap().text().unwrap();

    let document = Html::parse_document(&text);
    let id_selector =
        Selector::parse("video[data-video-id], a[data-json-href], iframe[src]").unwrap();

    // if (element.value().nam)

    for element in document.select(&id_selector) {
        let element_name = element.value().name();

        let video_id = match element_name {
            "video" => element.value().attr("data-video-id"),
            "a" => {
                // TODO
                let attr = element.value().attr("data-json-href");
                attr
            }
            "iframe" => {
                // TODO
                let attr = element.value().attr("src");
                attr
            }
            _ => None,
        };

        println!("{:?}", video_id);

        let mut data_url = [
            "https://api.svt.se/videoplayer-api/video/",
            video_id.unwrap(),
        ]
            .concat();
        println!("{:?}", data_url);

        let request = reqwest::get(&data_url);
        let text = request.unwrap().text().unwrap();

        let deserialized: SVTVideo = serde_json::from_str(&text).unwrap();
        println!("{:?}", deserialized);

        // FIXME Get url here to construct ffmpeg
    }

    // let mut dataUrl = "https://api.svt.se/videoplayer-api/video/" + videoId;
}

struct EventHandler {}

impl EventHandler {
    fn start_download(&self, url: String, progress: sciter::Value, done: sciter::Value) -> bool {
        thread::spawn(move || {
            visit_website(&url);

            let mut cmd = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(&["/C", "echo", &url])
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap()
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg("echo lol")
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap()
            };

            {
                let stdout = cmd.stdout.as_mut().unwrap();
                let stdout_reader = BufReader::new(stdout);
                let stdout_lines = stdout_reader.lines();

                for line in stdout_lines {
                    println!("Read: {:?}", line);
                    done.call(None, &make_args!(line.unwrap()), None).unwrap();
                }
            }
        });

        // cmd.wait().unwrap();
        return true;
    }
}
impl sciter::EventHandler for EventHandler {
    dispatch_script_call! {
        fn start_download(String, Value, Value);
    }
}

fn main() {
    let handler = EventHandler {};
    let mut frame = sciter::Window::new();
    frame.event_handler(handler);
    frame.load_file("html/main.html");
    frame.run_app();
}
