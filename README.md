# Polygon Drawing and Filling with Rust

This repository contains a Rust project for drawing and filling polygons using a framebuffer approach. It leverages basic computer graphics techniques to render filled polygons directly onto a simulated frame buffer, which can then be saved as BMP files.

## Features

- **Polygon Drawing**: Draw the outline of polygons based on a given set of vertices.
- **Polygon Filling**: Fill polygons using a scan-line fill algorithm, suitable for convex polygons.
- **BMP Output**: Save the rendered polygon as a BMP file to visually verify the output.

## Modules

The project is organized into several modules:

- `framebuffer`: Manages pixel data and drawing primitives like points and lines.
- `line_impl`: Implements the logic for drawing lines between points using Bresenham's line algorithm.
- `bmp.rs`: Handles the conversion of framebuffer data to BMP format.
- `main.rs`: Contains the main application logic, demonstrating how to draw and fill a triangle.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your machine. If not, you can install Rust via [rustup](https://rustup.rs/).

### Installation

Clone the repository to your local machine:

```bash
git clone https://your-repository-url.git
cd your-repository-directory

### Building and Running the Project

To build and run the project using Cargo, Rust's package manager and build system, follow these commands:

```bash
# Compile the project
cargo build

# Run the application
cargo run
