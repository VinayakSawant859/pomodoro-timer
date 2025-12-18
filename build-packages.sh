#!/bin/bash
# Build script for creating both DEB and RPM packages
# RPM is created via alien conversion from DEB since Tauri 2.0 RPM bundler hangs with RPM 6.x

set -e

echo "🔨 Building Pomodoro Timer packages..."
echo ""

# Build DEB using Tauri
echo "📦 Building DEB package..."
npm run tauri:build

DEB_FILE="src-tauri/target/release/bundle/deb/PomodoroTimer_1.0.0_amd64.deb"

if [ ! -f "$DEB_FILE" ]; then
    echo "❌ DEB build failed"
    exit 1
fi

echo "✅ DEB created: $DEB_FILE"
echo ""

# Convert DEB to RPM using alien
echo "🔄 Converting DEB to RPM using alien..."

if ! command -v alien &> /dev/null; then
    echo "❌ alien not found. Install with: sudo dnf install alien"
    exit 1
fi

cd src-tauri/target/release/bundle/deb
sudo alien --to-rpm --keep-version PomodoroTimer_1.0.0_amd64.deb

# Move RPM to standard location
mkdir -p ../rpm
RPM_FILE="pomodoro-timer-1.0.0-1.x86_64.rpm"

if [ -f "$RPM_FILE" ]; then
    sudo mv "$RPM_FILE" ../rpm/
    echo "✅ RPM created: ../rpm/$RPM_FILE"
else
    echo "❌ RPM conversion failed"
    exit 1
fi

cd ../../..

echo ""
echo "🎉 Build complete!"
echo ""
echo "📍 Package locations:"
echo "   DEB: $DEB_FILE"
echo "   RPM: src-tauri/target/release/bundle/rpm/$RPM_FILE"
echo ""
echo "💡 Installation:"
echo "   DEB: sudo apt install ./src-tauri/target/release/bundle/deb/PomodoroTimer_1.0.0_amd64.deb"
echo "   RPM: sudo rpm -Uvh ./src-tauri/target/release/bundle/rpm/$RPM_FILE"
