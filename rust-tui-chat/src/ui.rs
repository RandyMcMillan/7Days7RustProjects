use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

pub struct Ui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Ui {
    pub fn new() -> Result<Self, anyhow::Error> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn draw(&mut self, input: &str, messages: &[String]) -> Result<(), anyhow::Error> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());
            let messages_block = Block::default().title("Chat").borders(Borders::ALL);
            let input_block = Block::default().title("Input").borders(Borders::ALL);
            f.render_widget(messages_block, chunks[0]);
            f.render_widget(Paragraph::new(input), chunks[1]);
        })?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<(), anyhow::Error> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
