use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

pub(crate) trait Page {
    fn event(&mut self, evt: crossterm::event::Event) -> EventFeedback;
    fn header(&self) -> std::string::String;
}

pub(crate) trait PageWidget: Page + Widget {
    fn dispatch_render(&self, area: Rect, buf: &mut Buffer);
}

#[derive(Debug)]
pub(crate) enum EventFeedback {
    None,
    Exit,
}
