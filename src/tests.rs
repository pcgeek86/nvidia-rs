#[cfg(test)]
use super::*;
use ratatui::{buffer::Buffer, layout::Rect, style::Style};

#[test]
fn render() {
  let app = NvidiaApp::default();
  let mut buf = Buffer::empty(Rect::new(0,0,50,4));

  app.render(buf.area, &mut buf);


}

#[test]
fn handle_key_event() -> std::io::Result<()> {
  let mut app = NvidiaApp::default();
  app.handle_key_event(KeyCode::Char('q').into());
  assert!(app.exit);
  Ok(())
}