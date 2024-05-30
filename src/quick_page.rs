use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget, WidgetRef,
    },
};

use crate::{
    global::RETURN_CODE,
    page::{EventFeedback, Page, PageWidget},
};

#[derive(Debug, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub(crate) struct QuickItem {
    short: std::string::String,
    full: std::string::String,
    note: std::string::String,
}

impl QuickItem {
    pub fn new(s: String, f: String, n: String) -> Self {
        QuickItem {
            short: s,
            full: f,
            note: n,
        }
    }

    pub fn short(&self) -> &String {
        &self.short
    }

    pub fn full(&self) -> &String {
        &self.full
    }

    pub fn note(&self) -> &String {
        &self.note
    }
}

#[derive(Debug, Clone)]
pub(crate) struct QuickPage {
    name: String,
    size: usize,
    items: Vec<QuickItem>,
    curr_idx: usize,
    popup: bool,
}
impl Page for QuickPage {
    fn event(&mut self, evt: crossterm::event::Event) -> EventFeedback {
        match evt {
            Event::FocusGained => {
                println!("FocusGained");
                EventFeedback::None
            }
            Event::FocusLost => {
                println!("FocusLost");
                EventFeedback::None
            }
            Event::Key(k) => {
                match k.code {
                    KeyCode::Char('q') => {
                        let mut return_code = RETURN_CODE.lock().unwrap();
                        *return_code = 0;
                        EventFeedback::Exit
                    }
                    KeyCode::Char(' ') => {
                        self.flip_popup();
                        EventFeedback::None
                    }
                    KeyCode::Char('d') => {
                        self.delete();
                        EventFeedback::None
                    }
                    KeyCode::Char(_) => EventFeedback::None,
                    KeyCode::Backspace => EventFeedback::None,
                    KeyCode::Enter => {
                        //self.popup[self.selected_page_index] =
                        //    !self.popup[self.selected_page_index];
                        let mut return_code = RETURN_CODE.lock().unwrap();
                        *return_code = 0;
                        EventFeedback::Exit
                    }
                    KeyCode::Left => EventFeedback::None,
                    KeyCode::Right => EventFeedback::None,
                    KeyCode::Up => {
                        if self.curr_idx != 0 {
                            self.curr_idx -= 1;
                        }
                        EventFeedback::None
                    }
                    KeyCode::Down => {
                        if self.curr_idx != (self.size() - 1) {
                            self.curr_idx += 1;
                        }
                        EventFeedback::None
                    }
                    KeyCode::Home => EventFeedback::None,
                    KeyCode::End => EventFeedback::None,
                    KeyCode::PageUp => EventFeedback::None,
                    KeyCode::PageDown => EventFeedback::None,
                    KeyCode::Tab => EventFeedback::None,
                    KeyCode::BackTab => EventFeedback::None,
                    KeyCode::Delete => EventFeedback::None,
                    KeyCode::Insert => EventFeedback::None,
                    KeyCode::F(_) => EventFeedback::None,
                    KeyCode::Null => EventFeedback::None,
                    KeyCode::Esc => EventFeedback::None,
                    KeyCode::CapsLock => EventFeedback::None,
                    KeyCode::ScrollLock => EventFeedback::None,
                    KeyCode::NumLock => EventFeedback::None,
                    KeyCode::PrintScreen => EventFeedback::None,
                    KeyCode::Pause => {
                        // not space
                        // self.flip_popup();
                        EventFeedback::None
                    }
                    KeyCode::Menu => EventFeedback::None,
                    KeyCode::KeypadBegin => EventFeedback::None,
                    KeyCode::Media(_) => EventFeedback::None,
                    KeyCode::Modifier(_) => EventFeedback::None,
                }
            }
            Event::Mouse(m) => EventFeedback::None,
            Event::Paste(p) => EventFeedback::None,
            Event::Resize(x, y) => EventFeedback::None,
        }
    }

    fn header(&self) -> std::string::String {
        std::string::String::from("Quick")
    }
}

impl QuickPage {
    pub fn new(name: String, items: Vec<QuickItem>, curr_idx: usize, popup: bool) -> Self {
        Self {
            name,
            size: items.len(),
            items,
            curr_idx,
            popup,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn items_mut(&mut self) -> &mut Vec<QuickItem> {
        self.items.as_mut()
    }

    fn items(&self) -> &Vec<QuickItem> {
        self.items.as_ref()
    }
    fn item_idx(&self) -> usize {
        self.curr_idx
    }

    fn delete(&mut self) {
        let idx = self.item_idx();
        self.items_mut().remove(idx);
        self.size = self.items().len();
        if self.curr_idx == self.size() {
            self.curr_idx = self.size() - 1;
        }
    }

    fn flip_popup(&mut self) {
        self.popup = !self.popup;
    }

    fn is_popup(&self) -> bool {
        self.popup
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let page_title = "Quick";
        let page_block = Block::default()
            .title(page_title)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let page_text = Paragraph::new("test text/ raw/c.cc")
            .block(page_block)
            .alignment(Alignment::Center);

        page_text.render(area, buf);
    }

    fn render_list(&self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .items()
            .iter()
            .map(|i| ListItem::new(format!("  {}", (i.short().to_owned()))))
            .collect();

        let list = List::new(items)
            .direction(ratatui::widgets::ListDirection::TopToBottom)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(crate::global::SELECTED_STYLE_FG),
            )
            .highlight_symbol(">>")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        let mut state = ListState::default();
        state.select(Some(self.curr_idx));

        StatefulWidget::render(
            list,
            area.inner(&Margin {
                vertical: 1,
                horizontal: 2,
            }),
            buf,
            &mut state,
        );
    }

    fn render_preview(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Preview")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let idx = self.item_idx();
        let para = Paragraph::new(self.items().get(idx).unwrap().full().to_owned()).block(block);
        para.render(area, buf);
    }

    fn render_popup(&self, area: Rect, buf: &mut Buffer) {
        let popup_area = crate::global::centered_rect(60, 20, area);
        let idx = self.item_idx();
        let text = self.items().get(idx).unwrap().note();

        let popup = Paragraph::new(text.to_owned())
            .style(Style::default().bg(Color::White).fg(Color::Black))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Note")
                    .title_alignment(Alignment::Left),
            );
        Clear.render(popup_area, buf);
        popup.render(popup_area, buf)
    }
}

impl Widget for QuickPage {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        self.render_ref(area, buf);
    }
}

impl WidgetRef for QuickPage {
    #[doc = r" Draws the current state of the widget in the given buffer. That is the only method required"]
    #[doc = r" to implement a custom widget."]
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(4),
        ]);
        let [header_area, list_area, footer_area] = vertical.areas(area);
        self.render_header(header_area, buf);
        self.render_list(list_area, buf);
        self.render_preview(footer_area, buf);
        if self.is_popup() {
            self.render_popup(area, buf);
        }
    }
}
impl PageWidget for QuickPage {
    fn dispatch_render(&self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf);
    }
}
