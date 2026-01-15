# WriteApp

A minimalist, terminal-based writing application built with Rust and Ratatui. WriteApp provides a distraction-free environment for drafting, note-taking, and creative writing with Vim-style keybindings and markdown preview support.

## Features

### ✍️ Core Writing Experience
- **Clean TUI Interface**: Distraction-free writing environment in your terminal
- **Auto-save**: Your work is automatically saved as you write
- **Hard Wrap**: Text automatically wraps at 90 characters for better readability
- **Draft Management**: Create, edit, rename, and organize multiple drafts
- **Focus Mode**: Toggle focus mode to minimize distractions

### ⌨️ Vim Keybindings
- **Modal Editing**: Normal, Insert, and Visual modes just like Vim
- **Navigation**: Use `h`, `j`, `k`, `l` for cursor movement in Normal mode
- **Visual Selection**: Press `v` to enter Visual mode and select text
- **Quick Operations**: `i` to insert, `Esc` to return to Normal mode
- **Yank & Copy**: Copy selected text with `y` in Visual mode
- **Create from Selection**: Press `n` in Visual mode to create a new draft from selected text

### 📝 Markdown Support
- **Live Preview**: Toggle markdown preview with `p` key
- **Proper Rendering**: Headings, lists, emphasis, and code blocks rendered correctly
- **Side-by-side View**: Split screen showing raw text and formatted preview

### 📊 Flow Tracking
- **Writing Journal**: Track your writing sessions automatically
- **Session History**: Review past writing sessions with timestamps
- **Word Count Tracking**: Monitor your progress over time

### 🎯 Additional Features
- **Multiple Drafts**: Manage unlimited drafts in the Drafts view
- **Rename Drafts**: Press `r` in the drafts list or `Ctrl+r` while writing
- **Settings Persistence**: Preferences saved automatically
- **Configurable**: Toggle Vim mode and other settings in Settings view

## Installation

### Prerequisites
- Rust 1.70 or higher
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/timappledotcom/writeapp.git
cd writeapp

# Build the project
cargo build --release

# The binary will be available at target/release/writeapp
./target/release/writeapp
```

### Quick Run (Development)

```bash
cargo run
```

## Usage

### Getting Started

Launch WriteApp:
```bash
writeapp
```

You'll be greeted with a menu showing all available options.

### Navigation

**Main Menu:**
- `w` - Open Writing view
- `f` - View Flow (writing history)
- `d` - Browse Drafts
- `s` - Open Settings
- `q` - Quit application

### Writing View

#### With Vim Mode Enabled:
- **Normal Mode** (default):
  - `i` - Enter Insert mode
  - `v` - Enter Visual mode
  - `h/j/k/l` - Navigate left/down/up/right
  - `Ctrl+r` - Rename current draft
  - `Esc` - Return to menu

- **Insert Mode**:
  - Type normally
  - `Esc` - Return to Normal mode

- **Visual Mode**:
  - `h/j/k/l` - Extend selection
  - `y` - Yank (copy) selected text
  - `n` - Create new draft from selection
  - `Esc` - Return to Normal mode

#### Without Vim Mode:
- Type freely
- `Ctrl+s` - Save (auto-saves anyway)
- `Esc` - Return to menu

**Common Keys (all modes):**
- `p` - Toggle markdown preview
- `Tab` - Toggle focus mode

### Drafts View

- `↑/↓` or `j/k` - Navigate drafts list
- `Enter` - Open selected draft
- `r` - Rename selected draft
- `d` - Delete selected draft
- `n` - Create new draft
- `Esc` - Return to menu

### Flow History

- `↑/↓` or `j/k` - Navigate through sessions
- `Esc` - Return to menu

### Settings

- `↑/↓` or `j/k` - Navigate options
- `Enter` or `Space` - Toggle setting
- `Esc` - Return to menu and save

## Storage

All drafts and settings are stored in your system's standard documents directory:

- **Linux**: `~/Documents/WriteApp/`
- **macOS**: `~/Documents/WriteApp/`
- **Windows**: `%USERPROFILE%\Documents\WriteApp\`

### File Structure
```
WriteApp/
├── drafts/           # Your writing drafts
├── flow.json         # Writing session history
└── settings.json     # Application settings
```

## Keyboard Shortcuts Quick Reference

| Key | Action |
|-----|--------|
| `w` | Writing view (from menu) |
| `d` | Drafts view (from menu) |
| `f` | Flow history (from menu) |
| `s` | Settings (from menu) |
| `q` | Quit (from menu) |
| `Esc` | Back to menu / Normal mode |
| `p` | Toggle markdown preview |
| `Tab` | Toggle focus mode |
| `i` | Insert mode (Vim mode) |
| `v` | Visual mode (Vim mode) |
| `y` | Yank/copy (Visual mode) |
| `n` | New draft from selection (Visual mode) |
| `h/j/k/l` | Vim navigation |
| `Ctrl+r` | Rename draft (Writing view) |
| `r` | Rename draft (Drafts list) |
| `n` | New draft (Drafts list) |

## Configuration

Settings can be adjusted in the Settings view:
- **Vim Mode**: Enable/disable Vim-style keybindings
- **Focus Mode**: Toggle focus mode by default
- **Preview Mode**: Start with markdown preview enabled

## Tips

1. **Enable Vim Mode** for faster navigation and text manipulation
2. **Use Visual Mode** to quickly extract snippets into new drafts
3. **Toggle Preview** while writing markdown to see formatted output
4. **Review Flow History** to track your writing consistency
5. **Use Focus Mode** when you need maximum concentration

## Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests

## License

This project is open source. Feel free to use and modify as needed.

## Credits

Built with:
- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [tui-textarea](https://github.com/rhysd/tui-textarea) - Text editing widget
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdown parser

---

**Happy Writing! ✨**
