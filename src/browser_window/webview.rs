use napi_derive::*;
use wry::WebView;
use napi::Result;

#[napi]
pub struct Webview {
  pub(crate) webview: WebView,
}

#[napi]
impl Webview {
  #[napi]
  /// Launch a print modal for this window's contents.
  pub fn print(&self) -> Result<()> {
    self.webview.print().map_err(|e| {
      napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to print: {}", e),
      )
    })
  }

  #[napi]
  /// Set webview zoom level.
  pub fn zoom(&self, scale_facotr: f64) -> Result<()> {
    self.webview.zoom(scale_facotr).map_err(|e| {
      napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to zoom: {}", e),
      )
    })
  }

  #[napi]
  /// Hides or shows the webview.
  pub fn set_webview_visibility(&self, visible: bool) -> Result<()> {
    self.webview.set_visible(visible).map_err(|e| {
      napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to set webview visibility: {}", e),
      )
    })
  }

  #[napi]
  /// Whether the devtools is opened.
  pub fn is_devtools_open(&self) -> bool {
    self.webview.is_devtools_open()
  }

  #[napi]
  /// Opens the devtools.
  pub fn open_devtools(&self) {
    self.webview.open_devtools();
  }

  #[napi]
  /// Closes the devtools.
  pub fn close_devtools(&self) {
    self.webview.close_devtools();
  }

  #[napi]
  /// Loads the given URL.
  pub fn load_url(&self, url: String) -> Result<()> {
    self.webview.load_url(&url).map_err(|e| {
      napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to load URL: {}", e),
      )
    })
  }

  #[napi]
  /// Loads the given HTML content.
  pub fn load_html(&self, html: String) -> Result<()> {
    self.webview.load_html(&html).map_err(|e| {
      napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to load HTML: {}", e),
      )
    })
  }

  #[napi]
  /// Evaluates the given JavaScript code.
  pub fn evaluate_script(&self, js: String) -> Result<()> {
    self
      .webview
      .evaluate_script(&js)
      .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("{}", e)))
  }
}
