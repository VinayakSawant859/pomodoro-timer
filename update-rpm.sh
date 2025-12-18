#!/bin/bash
# Update script for upgrading installed Pomodoro Timer RPM package
# This script builds the latest DEB, converts to RPM, and upgrades the system package

set -e

echo "🔄 Pomodoro Timer RPM Update Script"
echo "===================================="
echo ""

# Check if RPM is installed
if ! rpm -qa | grep -q pomodoro-timer; then
    echo "❌ Pomodoro Timer not installed"
    echo "   Install with: sudo dnf install ./src-tauri/target/release/bundle/rpm/pomodoro-timer-1.0.0-1.x86_64.rpm"
    exit 1
fi

echo "📦 Current installation:"
rpm -qi pomodoro-timer | grep -E "Install Date|Version|Release"

echo ""
echo "🔨 Building latest packages..."

# Build new packages
./build-packages.sh

echo ""
echo "⬆️  Upgrading installed package..."

# Upgrade the RPM
cd src-tauri/target/release/bundle/rpm
sudo rpm -Uvh --force pomodoro-timer-1.0.0-1.x86_64.rpm

cd ../../..

echo ""
echo "✅ Update complete!"
echo ""
echo "📍 Verify update:"
rpm -qi pomodoro-timer | grep "Install Date"
echo ""
echo "🚀 Launch updated app:"
echo "   pomodoro-timer  (from terminal)"
echo "   or use application menu"
