# Pomodoro Timer

A modern, cross-platform Pomodoro Timer built with Svelte and Tauri. Features a clean UI with task management, customizable sessions, and audio notifications.

## Features

- **Timer Management**: Customizable work and break sessions (25+5 or 45+15 minute presets)
- **Task Integration**: Built-in todo list with add, edit, delete, and complete functionality
- **Audio Notifications**: Sound effects for timer completion and tick sounds
- **Theme Support**: Dark and light mode with smooth transitions
- **Session Tracking**: Monitor your productivity sessions
- **Cross-Platform**: Desktop app for Windows, macOS, and Linux

## Tech Stack

- **Frontend**: Svelte 5, TypeScript, CSS3
- **Backend**: Rust (Tauri 2.0)
- **Database**: SQLite (embedded)
- **Audio**: Rodio (Rust audio library)
- **Build Tool**: Vite

## Getting Started

### Prerequisites

- Node.js (v18 or later)
- Rust (latest stable)
- System dependencies for your platform:
  - **Linux**: `sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev libasound2-dev`
  - **Fedora**: `sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel alsa-lib-devel`

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/pomodoro-timer.git
   cd pomodoro-timer
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri:dev
   ```

4. Build for production:
   ```bash
   npm run tauri:build
   ```

## Usage

1. **Start a Session**: Choose from preset sessions or create custom timings
2. **Manage Tasks**: Click "Show Tasks" to access your todo list
3. **Audio Feedback**: Enable sound notifications for completion alerts
4. **Theme Toggle**: Switch between light and dark modes
5. **Track Progress**: Monitor completed sessions and tasks

## Project Structure

```
src/
├── lib/
│   ├── components/     # Svelte components
│   └── stores.ts      # State management
├── routes/            # Page routes
└── app.html          # HTML template

src-tauri/
├── src/
│   └── main.rs       # Rust backend
├── icons/            # App icons
└── tauri.conf.json   # Tauri configuration
```

## Development

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run tauri:dev` - Start Tauri development mode
- `npm run tauri:build` - Build desktop application

### Adding Features

1. **Frontend**: Add Svelte components in `src/lib/components/`
2. **Backend**: Extend Rust commands in `src-tauri/src/main.rs`
3. **Styling**: Use CSS variables defined in the theme system

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Commit changes (`git commit -am 'Add new feature'`)
4. Push to branch (`git push origin feature/new-feature`)
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Pomodoro Technique by Francesco Cirillo
- Built with [Tauri](https://tauri.app/) and [Svelte](https://svelte.dev/)