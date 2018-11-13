use services::svt::svt;
use services::tv4::tv4;

mod svt;
mod tv4;

enum Service {
  tv4,
  svt,
}

#[derive(Debug)]
pub struct Stream {
  pub filename: String,
  pub url: String,
  pub format: String,
}

fn determine_service(url: &str) -> Option<Service> {
  let re_tv4 = regex::Regex::new(r"^https?://(?:www\.)?tv4(?:play)?\.se/.*");
  let re_svt = regex::Regex::new(r"^https?://(?:www\.)?(?:svt|svtplay|oppetarkiv)\.se/");

  if re_tv4.unwrap().is_match(url) {
    return Some(Service::tv4);
  }
  if re_svt.unwrap().is_match(url) {
    return Some(Service::svt);
  }

  return None;
}

fn fetch_html(url: &str) -> String {
  let request = reqwest::get(url);
  request.unwrap().text().unwrap()
}

pub fn get_stream(url: &str) -> Result<Stream, &'static str> {
  let service = determine_service(url);

  if let Some(s) = service {
    let html = fetch_html(&url);

    Ok(match s {
      Service::svt => svt(&html).unwrap(),
      Service::tv4 => tv4(&url).unwrap(),
    })
  } else {
    Err("Panic at the disco! The url doesn't match any of the services supported.")
  }
}
