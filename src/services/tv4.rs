use services::Stream;

#[derive(Deserialize, Debug)]
struct Metadata<'a> {
  title: &'a str,
}

#[derive(Deserialize, Debug)]
struct TV4Asset<'a> {
  mediaUri: String,
  #[serde(borrow)]
  metadata: Metadata<'a>,
}

#[derive(Deserialize, Debug)]
struct PlaybackItem<'a> {
  manifestUrl: &'a str,
  #[serde(rename = "type")]
  format: &'a str,
}

#[derive(Deserialize, Debug)]
struct TV4Media<'a> {
  #[serde(borrow)]
  playbackItem: PlaybackItem<'a>,
}

pub fn tv4(url: &str) -> Result<Stream, &'static str> {
  let re = regex::Regex::new(r"^https?://(?:www\.)?tv4(?:play)?\.se/.*(?:-|/)(\d+)");

  let captures = re.unwrap().captures(url);
  let video_id = captures.unwrap().get(1).unwrap().as_str();

  // println!("Match {:?}", video_id);

  let data_url = format!("https://playback-api.b17g.net/asset/{}?service=tv4&device=browser&drm=widevine&protocol=hls%2Cdash", video_id);

  let request = reqwest::get(&data_url);
  let text = request.unwrap().text().unwrap();

  let asset: TV4Asset = serde_json::from_str(&text).unwrap();
  // println!("{:?}", asset);

  let request = reqwest::get(&format!("https://playback-api.b17g.net{}", &asset.mediaUri));
  let text = request.unwrap().text().unwrap();

  let media: TV4Media = serde_json::from_str(&text).unwrap();
  // println!("{:?}", media);

  Ok(Stream {
    filename: format!("{}.mp4", &asset.metadata.title),
    url: String::from(media.playbackItem.manifestUrl),
    format: String::from(media.playbackItem.format),
  })
}
