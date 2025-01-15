# Bevy OSM Viewer

Bevy OSM Viewer is a simple application which displays OSM data using bevy.

## Features

- Load and display OSM data from Overpass turbo
- Pan and zoom functionality
- Customizable rendering options

## Getting Started

### Prerequisites

- Rust and Cargo installed. You can get them from [rustup.rs](https://rustup.rs/).

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/bevy-osm-viewer.git
    cd bevy-osm-viewer
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Run the application:
    ```sh
    cargo run --release
    ```

## Usage

1. Run the application using `cargo run -- release`.
2. Press `U` to update what you are seeing
3. Use the mouse to pan and zoom around the map.
4. Press `U` again to update what you are seeing

## Up-coming features

- [ ] Smooth data download and dispaly
- [ ] Data filtering
- [ ] Rotation
- [ ] User location as base map lon and lat
- [ ] Custom user settings

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Bevy Engine](https://bevyengine.org/)
- [Bevy Pancam](https://github.com/johanhelsing/bevy_pancam)
- [Bevy prototype lyon](https://github.com/Nilirad/bevy_prototype_lyon)
- [GeoRust](https://georust.org/)
