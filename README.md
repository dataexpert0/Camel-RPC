# Camel – SumatraPDF Discord Rich Presence

Camel is a lightweight Windows background application written in Rust that updates your Discord Rich Presence based on the document currently open in **SumatraPDF**.

The application runs silently in the background and remains accessible only through a system tray icon.

## Features

- Displays the currently opened PDF name from SumatraPDF in Discord Rich Presence
- Runs in background (no console window)
- Persistent system tray icon
- Low resource usage
- No file access or modification

## Requirements

- Windows 10 or newer
- Discord Desktop running
- SumatraPDF installed

## Usage

1. Download the executable from the **Releases** section
2. Run `camel.exe`
3. A camel icon will appear in the system tray
4. Open a PDF in SumatraPDF
5. Discord Rich Presence updates automatically

To exit the application, right-click the tray icon and select **Exit**.

### Important Notes

- Camel **does not start SumatraPDF automatically**
- SumatraPDF must already be running for Rich Presence to work
- If SumatraPDF is closed, Camel stays idle in the system tray
- Only the window title is read — no files are accessed

## Manual Build

If you prefer to build it yourself:

```bash
cargo build --release
