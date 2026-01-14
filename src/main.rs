use anyhow::Result;
use app::App;
use clap::{Parser, Subcommand};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::{Duration, Instant};

mod app;
mod storage;
mod tui;
mod ui;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a distraction-free flow session
    Flow {
        /// Duration in minutes
        #[arg(long, default_value_t = 10)]
        time: u64,
    },
    /// View flow history
    FlowHistory,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut app = match cli.command {
        Some(Commands::Flow { time }) => App::with_flow_mode(time),
        Some(Commands::FlowHistory) => {
            let mut app = App::new();
            // Simulate 'h' from Menu to enter History mode properly
            app.handle_key_event(crossterm::event::KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
            app
        },
        None => App::new(),
    };

    let mut terminal = tui::init()?;
    let app_result = run_app(&mut terminal, &mut app);
    tui::restore()?;
    app_result
}

fn run_app(terminal: &mut tui::Tui, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    while !app.should_quit {
        terminal.draw(|f| ui::ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                app.handle_key_event(key);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }
    Ok(())
}
