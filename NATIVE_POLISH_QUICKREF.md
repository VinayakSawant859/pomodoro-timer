# Native Linux Polish - Quick Reference

## 🚀 Installation Command

```bash
sudo dnf install libappindicator-gtk3-devel webkit2gtk4.1-devel
```

## 📋 Changes Summary

### Dependencies Added
- **Cargo.toml**: `tray-icon` feature, `tauri-plugin-notification`
- **package.json**: `@tauri-apps/plugin-notification`

### Files Modified
1. `src-tauri/Cargo.toml` - Dependencies
2. `src-tauri/src/main.rs` - Tray setup & update_status command
3. `src-tauri/tauri.conf.json` - Notification plugin config
4. `src/lib/native.ts` - **NEW** - Native integration helpers
5. `src/lib/components/Timer.svelte` - Status updates & notifications

## 🎯 Features

### System Tray
- **Left Click**: Toggle window
- **Right Click**: Menu (Show, Quit)
- **Tooltip**: Live timer status

### Notifications
- Work complete: "🎉 Work Session Complete!"
- Break complete: "✨ Break Complete!"

### Window Title
- Format: `MM:SS - SessionType`
- Updates every second
- Visible in Super key overview & ALT+TAB

## 🔧 Usage in Code

```typescript
// Update window title and tray tooltip
import { updateStatus, formatTimerTitle } from "$lib/native";
const title = formatTimerTitle(14, 30, "Focus");
await updateStatus(title);

// Show notification
import { showNotification } from "$lib/native";
await showNotification("Title", "Body text");
```

## 📖 Full Documentation
See: `LINUX_NATIVE_POLISH.md`
