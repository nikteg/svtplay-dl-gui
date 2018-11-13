use scraper::Html;
use scraper::Selector;
use services::Stream;

#[derive(Deserialize, Debug)]
struct VideoReference<'a> {
  url: &'a str,
  format: &'a str,
}

#[derive(Deserialize, Debug)]
struct SVTVideo<'a> {
  programTitle: String,
  episodeTitle: String,
  #[serde(borrow)]
  videoReferences: Vec<VideoReference<'a>>,
}

pub fn svt(html: &str) -> Result<Stream, &'static str> {
  let document = Html::parse_document(html);
  let id_selector =
    Selector::parse("video[data-video-id], a[data-json-href], iframe[src]").unwrap();

  let element = document.select(&id_selector).next().unwrap();
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

  // println!("{:?}", video_id);

  let data_url = format!(
    "https://api.svt.se/videoplayer-api/video/{}",
    video_id.unwrap(),
  );
  // println!("{:?}", data_url);

  let request = reqwest::get(&data_url);
  let text = request.unwrap().text().unwrap();

  let video: SVTVideo = serde_json::from_str(&text).unwrap();
  // println!("{:?}", video);

  let hls_reference = video.videoReferences.iter().find(|&r| r.format == "hls");

  if let Some(r) = hls_reference {
    let filename = if video.programTitle != "" {
      format!("{} - {}.mp4", &video.programTitle, &video.episodeTitle)
    } else {
      format!("{}.mp4", &video.episodeTitle)
    };

    Ok(Stream {
      filename: filename,
      url: String::from(r.url),
      format: String::from(r.format),
    })
  } else {
    Err("Could not find hls stream")
  }
}
