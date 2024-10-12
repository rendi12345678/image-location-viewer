# Image Location Viewer

**Image Location Viewer** is a Rust program that extracts GPS coordinates from images using `exiftool` and opens the location in your default web browser using `xdg-open`.

## Installation

### Prerequisites

1. **Rust**: Make sure you have Rust installed on your system. You can install it using [rustup](https://rustup.rs/).

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. exiftool: You need to have exiftool installed. Install it using your package manager:

On Ubuntu/Debian:

```
sudo apt-get install exiftool
```

On Fedora:

```
sudo dnf install exiftool
```

On Arch:

```
paru -Sy perl-image-exiftool
```

3. xdg-open: This utility is usually pre-installed on Linux systems. It allows you to open a URL in the default web browser.

## Usage

```
git clone git@github.com:rendi12345678/image-location-viewer.git
cd image-location-viewer
cargo run <image url>
```
