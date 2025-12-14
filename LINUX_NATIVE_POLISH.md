# Native Linux (GNOME) Polish - Implementation Guide

## ✅ Implementation Complete

This document outlines the native integration features added for Linux GNOME desktop environment.

---

## 🔧 System Dependencies

### Fedora Linux

Install the required development headers for system tray support:

```bash
sudo dnf install libappindicator-gtk3-devel webkit2gtk4.1-devel
```

**What these packages provide:**
- `libappindicator-gtk3-devel` - System tray icon support for GNOME
- `webkit2gtk4.1-devel` - WebView rendering (Tauri requirement)

---

## 📦 Dependencies Added

### src-tauri/Cargo.toml

```toml
[dependencies]
tauri = { version = "2.0", features = ["tray-icon"] }  # Added tray-icon feature
tauri-plugin-notification = "2.0"                       # New dependency
```

---

## 🎯 Features Implemented

### 1. System Tray Icon 🖼️

**Location:** Top bar (GNOME Shell)

**Features:**
- **Left Click**: Toggle window show/hide
- **Right Click Menu**:
  - **Show**: Bring window to front
  - **Quit**: Exit application
- **Tooltip**: Displays current timer status (updates every second)

**Implementation:** `src-tauri/src/main.rs` - `setup()` function

### 2. Native Desktop Notifications 🔔

**Triggers:**
- Work session complete: "🎉 Work Session Complete!"
- Break complete: "✨ Break Complete!"

**Features:**
- Native GNOME notification system integration
- Permission handling (auto-request if needed)
- Custom title and body messages

**Implementation:** 
- Backend: `tauri-plugin-notification`
- Frontend: `src/lib/native.ts` - `showNotification()`

### 3. Dynamic Window Title 📝

**Format:** `MM:SS - SessionType`

**Examples:**
- `14:59 - Focus` (during work session)
- `04:30 - Break` (during break)
- `Pomodoro Timer` (when idle)

**Update Frequency:** Every second while timer is running

**GNOME Integration:**
- Press **Super (Windows) key** → See timer in window overview
- ALT+TAB switcher shows current time remaining
- Top bar window title displays countdown

**Implementation:** 
- Command: `src-tauri/src/main.rs` - `update_status()`
- Frontend: `src/lib/components/Timer.svelte` - updates in `$effect()` block

---

## 📁 Files Modified

### Backend (Rust)

1. **src-tauri/Cargo.toml**
   - Added `tray-icon` feature to tauri
   - Added `tauri-plugin-notification = "2.0"`

2. **src-tauri/src/main.rs**
   - Added tray icon initialization in `setup()`
   - Added menu items (Show, Quit)
   - Added tray icon event handlers (click, menu)
   - Added `update_status()` command for title/tooltip updates
   - Registered notification plugin

3. **src-tauri/tauri.conf.json**
   - Added notification plugin configuration

### Frontend (TypeScript/Svelte)

1. **src/lib/native.ts** (NEW FILE)
   - `updateStatus(text)` - Updates window title and tray tooltip
   - `showNotification(title, body)` - Shows native notifications
   - `formatTimerTitle(mins, secs, type)` - Formats title text

2. **src/lib/components/Timer.svelte**
   - Import native functions
   - Update title/tooltip every second in timer loop
   - Trigger notification when session completes
   - Reset title when timer stops

---

## 🚀 Usage

### Window Title Updates

The title updates automatically every second while the timer runs:

```typescript
// In Timer.svelte - $effect() block
const mins = Math.floor(timer.timeRemaining / 60);
const secs = timer.timeRemaining % 60;
const sessionType = timer.currentSession.type === "work" ? "Focus" : "Break";
const titleText = formatTimerTitle(mins, secs, sessionType);
await updateStatus(titleText);
```

### Showing Notifications

Automatically triggered on session completion:

```typescript
const notifTitle = timer.currentSession.type === "work" 
    ? "🎉 Work Session Complete!" 
    : "✨ Break Complete!";
const notifBody = timer.currentSession.type === "work"
    ? "Great work! Time for a well-deserved break."
    : "Break's over. Ready to focus again?";
await showNotification(notifTitle, notifBody);
```

### Custom Notifications (Manual)

You can also manually trigger notifications:

```typescript
import { showNotification } from "$lib/native";

await showNotification(
    "Custom Title", 
    "Custom message body"
);
```

---

## 🎨 GNOME-Specific Features

### Why No Dock Progress Bar?

GNOME Shell doesn't support Unity-style dock progress bars natively. Instead, we use:

1. **Window Title** - Shows in:
   - Top bar when window is focused
   - Super key overview
   - ALT+TAB switcher
   - GNOME Activities search

2. **Tray Tooltip** - Hover over tray icon to see timer

This provides better visibility than a dock progress bar would on GNOME.

### Tray Icon Behavior

- **Single Click (Left)**: Toggle window visibility
- **Right Click**: Show context menu
- **Hover**: Display current timer status (tooltip)

---

## 🔍 Testing

### Test Tray Icon
1. Build and run the app
2. Check top bar for Pomodoro icon
3. Left-click to hide/show window
4. Right-click to see menu
5. Hover to see tooltip

### Test Notifications
1. Start a 1-minute work session (custom)
2. Wait for completion
3. Verify notification appears
4. Check notification content

### Test Window Title
1. Start any timer session
2. Press **Super key** (Windows key)
3. Verify countdown visible in window overview
4. Use ALT+TAB to verify title updates

---

## 🛠️ Build & Install

```bash
# Install system dependencies first
sudo dnf install libappindicator-gtk3-devel webkit2gtk4.1-devel

# Build the application
cd /home/vinayak/Work/Pomodoro-By-Vinayak/pomodoro-app
npm run tauri:build

# Install the RPM
cd src-tauri/target/release/bundle/rpm
sudo rpm -Uvh --force "Pomodoro Timer-1.0.0-1.x86_64.rpm"
```

---

## 🎯 Benefits

### User Experience
- ✅ Check timer without opening window (tray tooltip)
- ✅ Quick window access from tray
- ✅ Native notification integration
- ✅ Timer visible in window switcher (Super/ALT+TAB)
- ✅ Minimizes to tray (doesn't clutter taskbar)

### Native Integration
- ✅ Follows GNOME HIG (Human Interface Guidelines)
- ✅ Uses system notification daemon
- ✅ Proper tray icon implementation
- ✅ Respects user notification permissions

---

## 🔧 Customization

### Change Notification Messages

Edit `src/lib/components/Timer.svelte`:

```typescript
const notifTitle = "Your Custom Title";
const notifBody = "Your custom message";
await showNotification(notifTitle, notifBody);
```

### Change Tray Menu Items

Edit `src-tauri/src/main.rs` in `setup()`:

```rust
let new_item = MenuItemBuilder::with_id("custom", "Custom Action")
    .build(app)?;

let menu = MenuBuilder::new(app)
    .items(&[&show_item, &new_item, &quit_item])
    .build()?;
```

### Change Title Format

Edit `src/lib/native.ts`:

```typescript
export function formatTimerTitle(minutes: number, seconds: number, sessionType: string): string {
    // Your custom format
    return `⏱️ ${minutes}:${seconds} | ${sessionType}`;
}
```

---

## 📚 References

- [Tauri Tray Icon Docs](https://v2.tauri.app/reference/javascript/api/namespacecore/#tray)
- [Tauri Notification Plugin](https://v2.tauri.app/plugin/notification/)
- [GNOME HIG - Notifications](https://developer.gnome.org/hig/patterns/feedback/notifications.html)
- [libappindicator Documentation](https://lazka.github.io/pgi-docs/AppIndicator3-0.1/index.html)

---

## ✨ Summary

Your Pomodoro Timer now has full native Linux GNOME integration:

- 🖼️ **System Tray**: Quick access and status updates
- 🔔 **Notifications**: Native desktop alerts
- 📝 **Dynamic Title**: See timer in window overview
- 🎯 **GNOME Polish**: Feels like a native application

Enjoy your polished Pomodoro experience! 🍅⏱️
