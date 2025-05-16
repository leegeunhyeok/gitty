use std::{
    env,
    sync::{
        mpsc::{channel, Sender, TryRecvError},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use mousefood::prelude::*;
use ratatui::{
    text::Span,
    widgets::{Paragraph, Wrap},
};

use crate::profile::Profile;

enum Status {
    Loading,
    Error(String),
    Ready(Profile),
}

pub struct App {
    username: Option<String>,
    status: Status,
}

impl Default for App {
    fn default() -> Self {
        Self {
            username: env::var("GITHUB_USERNAME").ok(),
            status: Status::Loading,
        }
    }
}

impl App {
    pub fn run<B: Backend>(mut self, mut terminal: Terminal<B>) -> Result<(), std::io::Error> {
        let (tx, rx) = channel();
        let tx = Arc::new(Mutex::new(tx));
        self.fetch_data(tx.lock().unwrap().clone());

        loop {
            self.update_status(rx.try_recv());

            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
        }
    }

    pub fn fetch_data(&mut self, tx: Sender<Option<Profile>>) {
        // TODO: Fetch GitHub profile data.
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(3));
            let _ = tx.send(Some(Profile::new(
                "username".to_string(),
                "dummy bio".to_string(),
                100,
                200,
            )));
        });
    }

    fn update_status(&mut self, data: Result<Option<Profile>, TryRecvError>) {
        if self.username.is_none() {
            self.status = Status::Error("No username found".to_string());
        }

        if let Ok(Some(data)) = data {
            self.status = Status::Ready(data);
        }
    }

    fn render_error(&self, area: Rect, buf: &mut Buffer, reason: &str) {
        let block = components::Block::error();
        Paragraph::new(reason)
            .centered()
            .wrap(Wrap { trim: true })
            .render(block.inner(area), buf);
        block.render(area, buf);
    }

    fn render_content(&self, area: Rect, buf: &mut Buffer) {
        let block = components::Block::basic(vec![
            Span::styled("Hello, ", Style::default()),
            Span::styled(
                self.username.as_ref().unwrap(),
                Style::default().fg(Color::Cyan),
            ),
        ]);
        let content_area = block.inner(area);

        match &self.status {
            Status::Loading => self.render_loading_content(content_area, buf),
            Status::Ready(_) => self.render_ready_content(content_area, buf),
            _ => unreachable!(),
        };

        block.render(area, buf);
    }

    fn render_loading_content(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ])
            .split(area);

        Paragraph::new("Loading...")
            .centered()
            .render(layout[1], buf);
    }

    fn render_ready_content(&self, area: Rect, buf: &mut Buffer) {
        let data = match &self.status {
            Status::Ready(data) => data,
            _ => unreachable!(),
        };

        Paragraph::new(format!("Data: {:#?}", data)).render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match &self.status {
            Status::Error(reason) => self.render_error(area, buf, reason.as_str()),
            _ => self.render_content(area, buf),
        }
    }
}

mod components {
    use ratatui::{
        style::{Color, Style},
        text::Span,
        widgets::{block::Title, Block as _Block, Borders},
    };

    pub struct Block;
    impl Block {
        pub fn basic<'a, T>(title: T) -> _Block<'a>
        where
            T: Into<Title<'a>>,
        {
            _Block::default().borders(Borders::ALL).title(title)
        }

        pub fn error<'a>() -> _Block<'a> {
            _Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title(Span::styled("Error", Style::default().fg(Color::Red)))
        }
    }
}
