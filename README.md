# FileQuerier 📂🔍🛠️

FileQuerier is an innovative Rust project designed to revolutionize the way you interact with Excel and CSV files. By leveraging the power of an in-memory SQLite database, FileQuerier allows you to query these files using SQL language, making data manipulation and analysis as seamless as never before. Our vision extends beyond traditional database usage; we aim to introduce a scripting implementation that enables these operations without any database dependency. Join us in shaping the future of file querying!

## Features

- **Load Excel & CSV Files into SQLite**: Import your data seamlessly into an in-memory database for fast and efficient querying.
- **SQL Query Execution**: Utilize the full power of SQL language to query your data without the hassle of database setup.
- **Future Scripting Implementation**: We're working on a scripting feature to query data without any database, making FileQuerier even more versatile.

## Getting Started

### Prerequisites

Before you dive into FileQuerier, ensure you have Rust installed on your system. If not, visit [Rust's official installation guide](https://www.rust-lang.org/tools/install) to get started.

### Installation

Clone the repository and build the project:

```bash
git clone https://github.com/cittaz/filequerier.git
cd filequerier
cargo build --release
```

### Usage

To start querying your Excel or CSV files, simply follow these steps:

```bash
# TODO
cargo run -- your-file-path.xlsx "SELECT * FROM your_sheet"
```

Replace `your-file-path.xlsx` with the path to your Excel/CSV file and `your_sheet` with the appropriate sheet name or CSV indicator.

## Contributing

FileQuerier thrives on community contributions. Whether it's feature development, bug fixes, or documentation improvements, all contributions are welcome. Please read our [CONTRIBUTING.md](https://github.com/cittaz/filequerier/CONTRIBUTING.md) for guidelines on how to contribute.

## Roadmap

- [x] Initial release: Load Excel and create table using SQLite.
- [x] Insert table data using SQLite.
- [ ] Update README.md.
- [ ] Implement CONTRIBUTING.md.
- [ ] Query the loaded data in SQLite.
- [ ] Implement correct sql data types.
- [ ] Query Excel files using SQLite.
- [ ] Implement multithreading.
- [ ] Implement GUI.
- [ ] Scripting implementation for database-free data querying.
- [ ] Expand file format support beyond Excel and CSV.
- [ ] Performance optimization for large datasets.

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgments

- SQLite for providing a robust in-memory database solution.
- The Rust community for continuous support and inspiration.
