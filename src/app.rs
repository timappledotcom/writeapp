use crate::storage::{self, FlowEntry, Settings};
use chrono::Utc;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::ListState;
use ratatui::style::Style;
use std::time::{Duration, Instant};
use tui_textarea::{TextArea, CursorMove};

const HARD_WRAP_LIMIT: usize = 90;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    Menu,
    Writing,
    Flow,
    FlowHistory,
    Settings,
    Drafts,
}

pub struct App<'a> {
    pub mode: Mode,
    pub should_quit: bool,
    pub textarea: TextArea<'a>,
    // Settings
    pub focus_mode_active: bool,
    pub preview_mode_active: bool,
    pub settings: Settings,
    
    // Drafts
    pub drafts: Vec<String>,
    pub drafts_state: ListState,
    pub current_draft_name: Option<String>,

    pub flow_duration: Duration,
    pub flow_start: Option<Instant>,
    pub flow_remaining: Duration,
    pub history_state: ListState,
    pub history: Vec<FlowEntry>,
    pub message: Option<String>,
    pub message_time: Option<Instant>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_cursor_line_style(Style::default()); // No highlight
        textarea.set_line_number_style(Style::default()); // No line numbers if any default? Actually default is none usually but good to be safe or maybe user wants them off.
        // tui-textarea default doesn't show line numbers but just in case.
        
        Self {
            preview_mode_active: false,
            focus_mode_active: false,
            settings: storage::Storage::load_settings().unwrap_or_default(),
            mode: Mode::Menu,
            should_quit: false,
            textarea,
            flow_duration: Duration::from_secs(600), // Default 10 min
            flow_start: None,
            flow_remaining: Duration::from_secs(600),
            history_state: ListState::default(),
            history: Vec::new(),
            drafts: Vec::new(),
            drafts_state: ListState::default(),
            current_draft_name: None,
            message: None,
            message_time: None,
        }
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_flow_mode(duration_mins: u64) -> Self {
        let mut app = Self::default();
        app.start_flow(duration_mins);
        app
    }

    pub fn tick(&mut self) {
        if self.mode == Mode::Flow {
            if let Some(start) = self.flow_start {
                let elapsed = start.elapsed();
                if elapsed >= self.flow_duration {
                    self.flow_remaining = Duration::ZERO;
                    self.end_flow(true); // Auto-save
                } else {
                    self.flow_remaining = self.flow_duration - elapsed;
                }
            }
        }
        
        // Clear message after 3 seconds
        if let Some(time) = self.message_time {
            if time.elapsed() > Duration::from_secs(3) {
                self.message = None;
                self.message_time = None;
            }
        }
    }

    pub fn start_flow(&mut self, duration_mins: u64) {
        self.mode = Mode::Flow;
        self.preview_mode_active = false;
        self.flow_duration = Duration::from_secs(duration_mins * 60);
        self.flow_remaining = self.flow_duration;
        self.flow_start = Some(Instant::now());
        self.textarea = TextArea::default(); 
        self.textarea.set_cursor_line_style(Style::default());
    }

    pub fn end_flow(&mut self, save: bool) {
        if save {
            self.save_flow_entry();
        }
        self.mode = Mode::Menu;
        self.flow_start = None;
        self.set_message("Flow session ended.");
    }

    fn save_flow_entry(&mut self) {
        let text = self.textarea.lines().join("\n");
        if text.trim().is_empty() {
            return;
        }
        let entry = FlowEntry {
            timestamp: Utc::now(),
            duration_minutes: (self.flow_duration.as_secs() / 60) as u32,
            text,
        };
        if let Err(e) = storage::Storage::save_flow_entry(entry) {
            self.set_message(format!("Error saving: {}", e));
        } else {
            self.set_message("Saved flow session.");
        }
    }
    
    pub fn set_message(&mut self, msg: impl Into<String>) {
        self.message = Some(msg.into());
        self.message_time = Some(Instant::now());
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match self.mode {
            Mode::Menu => match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Char('f') => self.start_flow(10), // Default 10
                KeyCode::Char('5') => self.start_flow(5),
                KeyCode::Char('s') => self.mode = Mode::Settings,
                KeyCode::Char('n') => {
                    self.mode = Mode::Writing;
                    self.textarea = TextArea::default();
                    self.textarea.set_cursor_line_style(Style::default());
                    self.preview_mode_active = false;
                    self.set_message("Writing mode"); 
                }
                KeyCode::Char('h') => {
                    self.mode = Mode::FlowHistory;
                    self.load_history();
                },
                KeyCode::Char('d') => {
                    self.mode = Mode::Drafts;
                    self.load_drafts();
                },
                _ => {}
            },
            Mode::Drafts => match key.code {
                KeyCode::Esc => self.mode = Mode::Menu,
                KeyCode::Down => self.next_draft(),
                KeyCode::Up => self.previous_draft(),
                KeyCode::Enter => {
                    if let Some(idx) = self.drafts_state.selected() {
                        if idx < self.drafts.len() {
                            let filename = &self.drafts[idx];
                            if let Ok(content) = storage::Storage::load_draft(filename) {
                                let mut textarea = TextArea::new(
                                    content.lines().map(|s| s.to_string()).collect()
                                );
                                textarea.set_cursor_line_style(Style::default());
                                self.textarea = textarea;
                                self.mode = Mode::Writing;
                                self.current_draft_name = Some(filename.clone());
                                self.set_message(format!("Loaded {}", filename));
                            } else {
                                self.set_message("Error loading draft");
                            }
                        }
                    }
                }
                KeyCode::Char('d') | KeyCode::Delete => {
                     if let Some(idx) = self.drafts_state.selected() {
                         if idx < self.drafts.len() {
                             let filename = &self.drafts[idx];
                             if let Err(e) = storage::Storage::delete_draft(filename) {
                                 self.set_message(format!("Error deleting: {}", e));
                             } else {
                                 self.set_message("Deleted draft");
                                 self.load_drafts();
                             }
                         }
                     }
                }
                _ => {}
            },
            Mode::Settings => match key.code {
                KeyCode::Esc | KeyCode::Char('q') => self.mode = Mode::Menu,
                KeyCode::Char('e') => {
                    // Toggle extension
                    if self.settings.default_extension == "txt" {
                        self.settings.default_extension = "md".to_string();
                    } else {
                        self.settings.default_extension = "txt".to_string();
                    }
                    if let Err(e) = storage::Storage::save_settings(&self.settings) {
                        self.set_message(format!("Error saving settings: {}", e));
                    }
                }
                _ => {}
            },
            Mode::Writing => {
                match key.code {
                    KeyCode::Esc => {
                        self.mode = Mode::Menu;
                        self.current_draft_name = None; // Reset so next New uses default
                    }
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                         let filename = if let Some(ref name) = self.current_draft_name {
                             name.clone()
                         } else {
                             let timestamp = Utc::now().format("%Y-%m-%d-%H%M%S");
                             format!("draft_{}.{}", timestamp, self.settings.default_extension)
                         };
                         
                         if let Err(e) = storage::Storage::save_draft(&filename, &self.textarea.lines().join("\n")) {
                             self.set_message(format!("Error saving: {}", e));
                         } else {
                             self.current_draft_name = Some(filename.clone());
                             self.set_message(format!("Saved {}", filename));
                         }
                    }
                    KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.focus_mode_active = !self.focus_mode_active;
                        let msg = if self.focus_mode_active { "Focus Mode ON" } else { "Focus Mode OFF" };
                        self.set_message(msg);
                    }
                    KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                         self.preview_mode_active = !self.preview_mode_active;
                         let msg = if self.preview_mode_active { "Preview ON" } else { "Preview OFF" };
                         self.set_message(msg);
                    }
                    _ => { 
                         // Only input if not in preview mode (view only)
                         if !self.preview_mode_active {
                            self.textarea.input(key); 
                            self.check_wrap();
                         }
                    }
                }
            },
            Mode::Flow => {
                match key.code {
                    KeyCode::Esc => self.end_flow(true),
                    _ => { 
                        self.textarea.input(key); 
                        self.check_wrap();
                    }
                }
            },
            Mode::FlowHistory => {
                match key.code {
                    KeyCode::Esc => self.mode = Mode::Menu,
                    KeyCode::Down => self.next_history(),
                    KeyCode::Up => self.previous_history(),
                    KeyCode::Enter => {
                        // Load selected into writing?
                        if let Some(idx) = self.history_state.selected() {
                            if idx < self.history.len() {
                                let mut textarea = TextArea::new(
                                    self.history[idx].text.lines().map(|s| s.to_string()).collect()
                                );
                                textarea.set_cursor_line_style(Style::default());
                                self.textarea = textarea;
                                self.mode = Mode::Writing;
                                self.set_message("Loaded history entry");
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn load_history(&mut self) {
        match storage::Storage::load_flow_history() {
            Ok(h) => {
                self.history = h;
                if !self.history.is_empty() {
                    self.history_state.select(Some(0));
                } else {
                    self.history_state.select(None);
                }
            },
            Err(e) => self.set_message(format!("Failed to load history: {}", e)),
        }
    }

    fn load_drafts(&mut self) {
        match storage::Storage::list_drafts() {
            Ok(d) => {
                self.drafts = d;
                if !self.drafts.is_empty() {
                    self.drafts_state.select(Some(0));
                } else {
                    self.drafts_state.select(None);
                }
            },
            Err(e) => self.set_message(format!("Failed to load drafts: {}", e)),
        }
    }

    fn next_history(&mut self) {
        let i = match self.history_state.selected() {
            Some(i) => {
                if i >= self.history.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.history_state.select(Some(i));
    }

    fn previous_history(&mut self) {
        let i = match self.history_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.history.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.history_state.select(Some(i));
    }

    fn next_draft(&mut self) {
        let i = match self.drafts_state.selected() {
            Some(i) => {
                if !self.drafts.is_empty() {
                    if i >= self.drafts.len().saturating_sub(1) {
                        0
                    } else {
                        i + 1
                    }
                } else {
                    0
                }
            }
            None => 0,
        };
        self.drafts_state.select(Some(i));
    }

    fn previous_draft(&mut self) {
        let i = match self.drafts_state.selected() {
            Some(i) => {
                if !self.drafts.is_empty() {
                    if i == 0 {
                        self.drafts.len().saturating_sub(1)
                    } else {
                        i - 1
                    }
                } else {
                    0
                }
            }
            None => 0,
        };
        self.drafts_state.select(Some(i));
    }

    fn check_wrap(&mut self) {
        let (row, col) = self.textarea.cursor();
        // Since lines() returns a reference to vector of strings, we can query it
        if let Some(line) = self.textarea.lines().get(row) {
            if line.len() > HARD_WRAP_LIMIT {
                 // Try to split at the last space before the limit
                 // We limit the search to the first HARD_WRAP_LIMIT + 5 chars to avoid scanning too far back if user just typed?
                 // Actually, just searching backwards from the end or cursor.
                 // Let's search from the end of the line (which is > LIMIT)
                 
                 // Find last space within the first LIMIT chars? Or just last space generally?
                 // If we find a space at index 95 (and limit is 90), that doesn't help wrapping at 90.
                 // We need a space <= 90.
                 
                 let split_limit = HARD_WRAP_LIMIT;
                 let search_slice = &line[..split_limit];
                 if let Some(space_idx) = search_slice.rfind(' ') {
                     // We found a space within the limit. 
                     // Move cursor there, delete it, insert newline.
                     // But we must be careful: moving cursor changes `row`, `col`.
                     
                     // 1. Move to space
                     self.textarea.move_cursor(CursorMove::Jump(row as u16, space_idx as u16));
                     // 2. Delete the space (character at cursor)
                     self.textarea.delete_next_char();
                     // 3. Insert newline
                     self.textarea.insert_newline();
                     
                     // 4. Restore cursor position if it was ahead of the split
                     // If original `col` was > `space_idx`, the cursor is now on the next line.
                     // New row = row + 1.
                     // New col = original_col - space_idx - 1 (since newline replaced space).
                     
                     if col > space_idx {
                         let new_row = row + 1;
                         let new_col = col.saturating_sub(space_idx).saturating_sub(1);
                         self.textarea.move_cursor(CursorMove::Jump(new_row as u16, new_col as u16));
                     }
                 }
            }
        }
    }
}
