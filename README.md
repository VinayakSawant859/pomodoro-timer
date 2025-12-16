# 🍅 Pomodoro Timer

A modern, cross-platform Pomodoro Timer built with Svelte 5 and Tauri 2.0. Features a sleek UI with task management, customizable sessions, audio notifications, Zen Mode with ambient sounds, and 9 dynamic themes.

![Pomodoro Timer](src-tauri/icons/icon.ico)

> **Built for productivity enthusiasts** - A beautiful, feature-rich Pomodoro Timer that helps you stay focused and organized with distraction-free Zen Mode.

## ✨ Features

### 🎯 Core Functionality
- **Timer Management**: Customizable work and break sessions (25+5 or 45+15 minute presets)
- **Task Integration**: Built-in todo list with add, edit, delete, and complete functionality
- **Audio Notifications**: Sound effects for timer completion and tick sounds
- **Session Tracking**: Monitor your productivity sessions with visual progress indicators
- **Statistics Dashboard**: Track daily and weekly pomodoro completion with charts

### 🧘 Zen Mode & Ambient Sounds
- **Distraction-Free Mode**: Focus-only interface that hides tasks and shows only the timer
- **Ambient Sound Library**: 6 curated white noise tracks with album cover art
  - Boiler Ambient Noise
  - Ambi Val
  - Cumulus Clouds
  - Natural Sample
  - Quantum White
  - Tranquil White Noise
- **Atmospheric Visualizer**: 7-bar animated visualizer with slow, organic wave motion
- **Album Cover Display**: Beautiful cover art for each track with automatic fallback
- **Browse Interface**: Smooth slide-in panel to select ambient sounds

### 🎨 User Experience & Theming
- **9 Dynamic Themes**: Professionally designed themes for every mood
  - 🌙 **Dark**: Classic dark mode for low-light environments
  - ☀️ **Light**: Clean, bright interface for daytime work
  - 📚 **Academia**: Warm, scholarly browns and tans
  - 🌸 **Sakura**: Soft pink cherry blossom aesthetic
  - ☕ **Coffee**: Rich, warm coffee shop vibes
  - 🌲 **Forest**: Calming green nature tones
  - 🔥 **Flame**: Bold orange and red energy
  - 🎌 **Anime**: Vibrant purple anime-inspired palette
- **Smart Theme Detection**: Auto-adapts to system preferences on launch
- **Smooth Transitions**: Animated theme switching with color interpolation
- **Custom Icon Design**: Professional custom SVG icons throughout
  - Tomato, Timer, Fire, Goal, Growth, Headphone, Tick, Coffee, Work
  - Scalable vector graphics for crisp display at any size
  - Theme-aware coloring using CSS variables

### 🖥️ Platform Integration
- **Cross-Platform**: Native desktop app for Windows, macOS, and Linux
- **System Integration**: Appears in application menus with custom icon
- **Standalone Executable**: No need for development environment after build
- **Desktop Shortcuts**: One-click access from desktop or taskbar
- **Native Audio**: Rust-based Rodio audio engine for reliable playback

## Tech Stack

- **Frontend**: Svelte 5 (with Runes), TypeScript, CSS3
- **Backend**: Rust (Tauri 2.0)
- **Database**: SQLite (embedded)
- **Audio**: Rodio (Rust audio library with MP3 support)
- **Build Tool**: Vite 5
- **Charts**: Chart.js for statistics visualization

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

The dynamic theming system uses CSS custom properties with 9 professionally designed themes:

**Available Themes:**
- **Dark** - Classic dark mode for low-light work
- **Light** - Clean, bright interface for daytime
- **Academia** - Warm scholarly browns and tans
- **Sakura** - Soft pink cherry blossom aesthetic
- **Coffee** - Rich coffee shop ambiance
- **Forest** - Calming green nature tones
- **Flame** - Bold orange and red energy
- **Anime** - Vibrant purple anime-inspired palette

**Features:**
- **Auto Detection**: Follows system dark/light mode preference on launch
- **Manual Toggle**: Theme switcher in the application header
- **Smooth Transitions**: Animated theme changes with CSS transitions
- **Consistent Design**: All colors derived from CSS variables for perfect theme integration
- **Color Mixing**: Uses `color-mix()` for hover states and derived colors

**Theme Variables:**
```css
:root {
  --background-color: #ffffff;
  --surface-color: #f8f9fa;
  --text-color: #1a1a1a;
  --text-secondary: #6c757d;
  --primary-color: #6366f1;
  --border-color: #e9ecef;
  /* ... more variables */
}
```

### 🧘 Zen Mode Features

**Ambient Sound System:**
- **Cover Art Display**: Each track shows its album artwork with subtle overlay
- **Atmospheric Visualizer**: 7 animated bars with slow wave motion (2.6s-3.4s cycles)
- **Browse Interface**: Smooth slide-in panel (220ms transition) with 3×2 grid
- **Smart Fallback**: Gracefully falls back to music icon if cover art fails to load
- **Theme Integration**: All colors use CSS variables for consistent appearance

**User Experience:**
- **Distraction-Free**: Hides task list and shows only timer + ambient sounds
- **Quick Access**: One-click toggle to enter/exit Zen Mode
- **Persistent State**: Remembers last selected track and playing state
- **Micro-Animations**: Polished 150-220ms transitions with cubic-bezier easing

## 📱 Application Showcase

### 🎨 Theme Gallery
The app includes 9 professionally designed themes:

| Theme | Description | Best For |
|-------|-------------|----------|
| **Dark** | Classic dark interface | Evening work, low-light environments |
| **Light** | Clean, bright interface | Daytime productivity, bright spaces |
| **Academia** | Warm browns and tans | Study sessions, reading |
| **Sakura** | Soft pink aesthetic | Creative work, gentle focus |
| **Coffee** | Rich coffee tones | Morning routines, cozy work |
| **Forest** | Calming green | Nature lovers, meditation |
| **Flame** | Bold orange/red | High-energy sessions |
| **Anime** | Vibrant purple | Fun, energetic work |

### 🧘 Zen Mode
- **Ambient Sound Library**: 6 curated white noise tracks with album covers
- **Visual Feedback**: 7-bar atmospheric visualizer with organic wave animation
- **Focus-First Design**: Timer-centric interface with minimal distractions
- **Browse Panel**: Smooth slide-in interface to select ambient sounds

### 🎯 Custom Icon Set
The app features custom-designed SVG icons:
- **Timer Icons**: Tomato, Timer, Fire for timer controls
- **Productivity**: Goal, Growth, Tick for achievements
- **UI Elements**: Headphone for audio, Coffee/Work for session types
- **Vector Quality**: Crisp at any resolution
- **Theme Aware**: Colors adapt to current theme

### Desktop Integration
- **Application Menu**: Appears with custom icon in system application launchers
- **Taskbar/Dock**: Shows branded icon when running
- **Desktop Shortcut**: One-click access with recognizable pomodoro icon
- **Native Notifications**: System-integrated timer completion alerts

## 🚀 Usage

### Basic Workflow
1. **Start a Session**: Choose from preset sessions (25+5, 45+15) or create custom timings
2. **Manage Tasks**: Click "Show Tasks" to access your integrated todo list
3. **Enable Zen Mode**: Click the 🧘 button to enter distraction-free mode with ambient sounds
4. **Select Ambient Sound**: In Zen Mode, click "Browse" to choose from 6 white noise tracks
5. **Audio Feedback**: Enable sound notifications for completion alerts
6. **Theme Selection**: Switch between 9 themes using the theme toggle in the header
7. **View Statistics**: Click the 📊 icon to see your productivity charts

### 🎨 Personalization
- **Theme Switching**: Choose from 9 professionally designed themes (Dark, Light, Academia, Sakura, Coffee, Forest, Flame, Anime)
- **Zen Mode**: Toggle distraction-free mode with ambient sound support
- **Custom Sessions**: Adjust work/break durations to match your workflow
- **Task Management**: Organize your work with the integrated todo list
- **Sound Preferences**: Choose from 6 ambient sound tracks or work in silence

### 🧘 Zen Mode Guide
1. Click the 🧘 (Monk Mode) button to enter Zen Mode
2. The interface simplifies to show only the timer and ambient sounds
3. Click "Browse" to see the 6 available ambient sound tracks with cover art
4. Select a track to start playing—the browse panel auto-closes
5. Use the play/pause button to control ambient sound playback
6. Watch the 7-bar visualizer animate with the sound
7. Exit Zen Mode anytime by clicking the 🧘 button again

## Project Structure

```
src/
├── lib/
│   ├── components/          # Svelte components
│   │   ├── Timer.svelte            # Main timer component
│   │   ├── TaskManager.svelte      # Task list with CRUD operations
│   │   ├── ThemeToggle.svelte      # 9-theme selector
│   │   ├── WhiteNoise.svelte       # Ambient sound player with visualizer
│   │   ├── Statistics.svelte       # Chart.js statistics dashboard
│   │   ├── SessionProgress.svelte  # Visual session progress
│   │   ├── CompletionDialog.svelte # Session completion modal
│   │   └── DailySummary.svelte     # Daily stats summary
│   └── stores.ts            # Svelte 5 state stores with runes
├── routes/
│   └── +page.svelte         # Main application page
└── app.html                 # HTML template

src-tauri/
├── src/
│   └── main.rs              # Rust backend with audio commands
├── icons/                   # App icons (PNG, ICO, ICNS)
│   ├── icon.ico
│   ├── 32x32.png
│   ├── 128x128.png
│   └── ...
└── tauri.conf.json          # Tauri configuration

static/
├── icons/                   # Custom SVG icons
│   ├── tomato.svg, timer.svg, fire.svg
│   ├── goal.svg, growth.svg, tick.svg
│   ├── headphone.svg, coffee.svg, work.svg
│   └── ...
└── white-noise/             # Ambient sound library
    ├── boiler-ambient-noise.mp3/.jpg
    ├── Ambi-Val.mp3/.jpg
    ├── Cumulus-Clouds.mp3/.jpg
    ├── Natural-Sample-Makers.mp3/.jpg
    ├── Quantum-White.mp3/.jpg
    └── Tranquil-White-Noise.mp3/.jpg
```

## Development

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run tauri:dev` - Start Tauri development mode
- `npm run tauri:build` - Build desktop application

### Adding Features

1. **Frontend Components**: Add Svelte components in `src/lib/components/`
2. **Rust Commands**: Extend backend in `src-tauri/src/main.rs`
3. **New Themes**: Add theme definitions in the CSS theme system
4. **Custom Icons**: Add SVG icons to `static/icons/`
5. **Ambient Sounds**: Add MP3 + JPG pairs to `static/white-noise/`
6. **Styling**: Use CSS variables for theme-aware colors

### Key Technologies

- **Svelte 5 Runes**: Modern reactivity with `$state`, `$derived`, `$effect`
- **Tauri 2.0 Commands**: Rust functions invoked from frontend via `invoke()`
- **Rodio Audio**: Rust audio playback with streaming support
- **CSS Variables**: Theme system with `color-mix()` for derived colors
- **Chart.js**: Interactive statistics visualization

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