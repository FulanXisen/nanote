mod dot_toml;
mod global;
mod page;
mod quick_page;
mod utils;

use anyhow::{Ok, Result};
use clap::{self, arg, Command};
use color_eyre::config::HookBuilder;
use global::RETURN_CODE;
use page::PageWidget;
use quick_page::{QuickItem, QuickPage};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    widgets::Widget,
    Terminal,
};
use std::{
    boxed,
    io::{self, stdout, Write},
    time::Duration,
    vec,
};

use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

struct Nav {
    curr_idx: usize,
    pages: Vec<Box<dyn PageWidget>>,
}

impl Widget for &Nav {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let idx = self.curr_idx;
        let page = self.pages.get(idx).unwrap();
        page.as_ref().dispatch_render(area, buf);
    }
}

impl Nav {
    fn new(pages: Vec<Box<dyn PageWidget>>) -> Self {
        Self { pages, curr_idx: 0 }
    }

    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
        loop {
            self.draw(&mut terminal)?;
            if crossterm::event::poll(Duration::from_millis(1000))? {
                let idx = self.curr_idx;
                match self.pages.get_mut(idx).unwrap().event(event::read()?) {
                    page::EventFeedback::None => (),
                    page::EventFeedback::Exit => return io::Result::Ok(()),
                }
            }
        }
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        std::io::Result::Ok(())
    }
}

fn read_quick_from_config(toml_data: &toml::Value) -> std::vec::Vec<QuickItem> {
    match toml_data.get("quick") {
        Some(quick_array) => match quick_array.as_array() {
            Some(items) => items
                .iter()
                .filter_map(|item| item.clone().try_into().ok())
                .collect(),
            None => Vec::new(),
        },
        None => Vec::new(),
    }
}

fn main_tui(path: &String) -> Result<()> {
    let dot_toml = crate::dot_toml::read_total_from_dot_toml(path).unwrap();
    let quick_items = dot_toml.quick;
    let quick_page = QuickPage::new(String::from("Quick"), quick_items, 0, false);
    let terminal = init_terminal()?;

    init_error_hooks()?;
    let mut nav = Nav::new(vec![boxed::Box::new(quick_page)]);

    nav.run(terminal)?;

    restore_terminal()?;
    // let preset = if nav.curr_idx == 0 {
    //     let item_idx = nav.selected_item_index[nav.selected_page_index];
    //     nav.command_items()[item_idx].full()
    // } else {
    //     "no no no "
    // };
    // let file_path = "/Users/fanyx/tools/nav/clipboard";
    // let mut file = File::create(file_path)?;
    // file.write_all(preset.as_bytes())?;
    let return_code = RETURN_CODE.lock().unwrap();
    if *return_code == 0 {
        color_eyre::Result::Ok(())
    } else {
        std::process::exit(*return_code);
    }
}

fn main_add(path: &String) -> Result<()> {
    if let Some(cmd) = utils::read_last_terminal_command() {
        io::stdout().flush().unwrap();
        let short = utils::prompt_user("short:")?;
        let note = utils::prompt_user("note:")?;
        let new_item = QuickItem::new(short, cmd, note);
        let mut dot_toml = crate::dot_toml::read_total_from_dot_toml(path)?;
        dot_toml.quick.push(new_item);
        crate::dot_toml::write_total_to_dot_toml(&dot_toml, path)?;
    }
    Ok(())
}

fn main_install() {}

fn main_uninstall() {}

fn main() -> Result<()> {
    let path = "/Users/fanyx/tools/nav/nav.toml".to_string();
    let matches = clap::command!()
        .subcommand(Command::new("add").about("add last terminal command to nav's quick page"))
        .get_matches();

    if matches.subcommand_matches("add").is_some() {
        main_add(&path)?;
    } else {
        main_tui(&path)?;
    }
    let return_code = RETURN_CODE.lock().unwrap();
    if *return_code == 0 {
        color_eyre::Result::Ok(())
    } else {
        std::process::exit(*return_code);
    }
}

fn init_error_hooks() -> Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;

    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info);
    }));
    Ok(())
}

fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
