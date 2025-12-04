# 🍅 Pomodoro Timer

A modern, cross-platform Pomodoro Timer built with Svelte and Tauri. Features a sleek UI with task management, customizable sessions, audio notifications, and dynamic theming with a custom icon design.

![Pomodoro Timer](src-tauri/icons/icon.png)

> **Built for productivity enthusiasts** - A beautiful, feature-rich Pomodoro Timer that helps you stay focused and organized.

## ✨ Features

### 🎯 Core Functionality
- **Timer Management**: Customizable work and break sessions (25+5 or 45+15 minute presets)
- **Task Integration**: Built-in todo list with add, edit, delete, and complete functionality
- **Audio Notifications**: Sound effects for timer completion and tick sounds
- **Session Tracking**: Monitor your productivity sessions

### 🎨 User Experience
- **Dynamic Theming**: Beautiful dark and light modes with smooth transitions
  - Auto-adapts to system preferences
  - Manual theme toggle with instant switching
  - Consistent color schemes across all components
- **Custom Icon Design**: Professional pomodoro-themed icon for desktop integration
  - Scalable vector-based design
  - Platform-optimized formats (PNG, ICO, ICNS)
  - Integrated with system application menus

### 🖥️ Platform Integration
- **Cross-Platform**: Native desktop app for Windows, macOS, and Linux
- **System Integration**: Appears in application menus with custom icon
- **Standalone Executable**: No need for development environment after build
- **Desktop Shortcuts**: One-click access from desktop or taskbar

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
   git clone https://github.com/VinayakSawant859/pomodoro-timer.git
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

### 🎨 Custom Icon Setup

The app includes a custom pomodoro-themed icon. To customize it further:

1. Replace `src-tauri/icons/icon.ico` with your design
2. Regenerate all icon formats:
   ```bash
   # Convert to required formats (requires ImageMagick)
   cd src-tauri/icons
   magick icon.ico -resize 32x32 32x32.png
   magick icon.ico -resize 128x128 128x128.png  
   magick icon.ico -resize 256x256 128x128@2x.png
   ```
3. Rebuild the application:
   ```bash
   npm run tauri:build
   ```

### 🌙 Theme System

The dynamic theming system uses CSS custom properties:

- **Auto Detection**: Follows system dark/light mode preference
- **Manual Toggle**: Theme switcher in the application header
- **Smooth Transitions**: Animated theme changes with CSS transitions
- **Consistent Design**: Unified color palette across all components

**Theme Variables:**
```css
:root {
  --bg-primary: #ffffff;
  --text-primary: #333333;
  --accent: #e74c3c;
  /* ... more variables */
}

[data-theme="dark"] {
  --bg-primary: #1a1a1a;
  --text-primary: #ffffff;
  --accent: #ff6b6b;
  /* ... dark mode overrides */
}
```

## 📱 Application Showcase

### Icon Design
- **Custom Pomodoro Icon**: Professional tomato-themed design optimized for all platforms
- **Multiple Formats**: PNG, ICO, ICNS for Windows, Linux, and macOS compatibility
- **Scalable Design**: Looks crisp at all sizes from 16x16 to 512x512 pixels

### Theme Demonstrations
| Light Mode | Dark Mode |
|------------|-----------|
| Clean, bright interface with red accent | Elegant dark interface with warm red highlights |
| Perfect for daytime productivity | Ideal for evening work sessions |
| Easy on the eyes in bright environments | Reduces eye strain in low light |

### Desktop Integration
- **Application Menu**: Appears with custom icon in system application launchers
- **Taskbar/Dock**: Shows branded icon when running
- **Desktop Shortcut**: One-click access with recognizable pomodoro icon
- **Native Notifications**: System-integrated timer completion alerts

## 🚀 Usage

### Basic Workflow
1. **Start a Session**: Choose from preset sessions (25+5, 45+15) or create custom timings
2. **Manage Tasks**: Click "Show Tasks" to access your integrated todo list
3. **Audio Feedback**: Enable sound notifications for completion alerts and ambient sounds
4. **Theme Toggle**: Switch between light and dark modes using the theme button
5. **Track Progress**: Monitor completed sessions and task completion

### 🎨 Personalization
- **Theme Switching**: Click the theme toggle in the header for instant light/dark mode switching
- **Custom Icon**: The app uses a professional pomodoro-themed icon visible in:
  - System application menu
  - Taskbar/dock when running
  - Desktop shortcuts
  - Window title bars

### 📱 Installation Methods
- **Development**: `npm run tauri:dev` for testing
- **Standalone**: `npm run tauri:build` creates installer packages
- **System Integration**: Install via generated .rpm/.deb/.msi files for full desktop integration

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