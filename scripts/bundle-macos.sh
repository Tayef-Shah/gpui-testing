#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."

# Homebrew rustup may be installed without Cargo proxies on PATH.
if ! command -v cargo >/dev/null || ! command -v rustc >/dev/null; then
    toolchain_bin="$(dirname "$(rustup which cargo)")"
    export PATH="$toolchain_bin:$PATH"
fi
if [ "$(uname -s)" != "Darwin" ]; then
    printf "This bundling script requires macOS.\n" >&2
    exit 1
fi
cargo build --release --locked

app_path="$PWD/dist/GPUI Pong.app"
mkdir -p "$app_path/Contents/MacOS" "$app_path/Contents/Resources"
cp target/release/gpui-testing "$app_path/Contents/MacOS/gpui-testing"
chmod 755 "$app_path/Contents/MacOS/gpui-testing"
cp assets/Pong.icns "$app_path/Contents/Resources/Pong.icns"
cat > "$app_path/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key><string>gpui-testing</string>
    <key>CFBundleIdentifier</key><string>local.gpui-testing.pong</string>
    <key>CFBundleName</key><string>GPUI Pong</string>
    <key>CFBundleDisplayName</key><string>GPUI Pong</string>
    <key>CFBundleIconFile</key><string>Pong.icns</string>
    <key>CFBundlePackageType</key><string>APPL</string>
    <key>CFBundleShortVersionString</key><string>0.1.0</string>
    <key>CFBundleVersion</key><string>1</string>
    <key>CFBundleInfoDictionaryVersion</key><string>6.0</string>
    <key>LSMinimumSystemVersion</key><string>11.0</string>
    <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
PLIST
printf 'APPL????' > "$app_path/Contents/PkgInfo"
plutil -lint "$app_path/Contents/Info.plist"
# Ad-hoc signing is suitable for this locally built app.
codesign --force --sign - "$app_path"
codesign --verify --strict "$app_path"
printf '\nCreated: %s\nLaunch: open "%s"\n' "$app_path" "$app_path"
