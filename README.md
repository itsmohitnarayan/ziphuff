# Zip Huff
A ZIP extractor and compressor made in Rust using the Huffman coding algorithm.
A GUI-based application for file compression and extraction using the Huffman coding algorithm, built with Rust and Egui.

## Overview

This project provides a graphical user interface for compressing and extracting text files using the Huffman coding algorithm. It supports both word-based and character-based compression modes.

## Features

- **File Compression**: Compress text files using Huffman coding.
- **File Extraction**: Extract compressed files back to their original form.
- **Two Modes**: Supports both word-based and character-based compression.
- **User-Friendly GUI**: Easy-to-use graphical interface built with Egui.
- **Progress and Timing**: Displays progress and elapsed time for operations.

## Screenshots

<img width="402" alt="zip huff" src="https://github.com/itsmohitnarayan/ziphuff/assets/68772712/e21407c1-e2d1-462e-a6a2-70e7da4006fa">
<img width="403" alt="zip huff2" src="https://github.com/itsmohitnarayan/ziphuff/assets/68772712/6be63f20-3ea9-4e78-a98f-75a8b2450a31">

## Installation

### Prerequisites

- Rust: Install from [rust-lang.org](https://www.rust-lang.org/)

### Clone the Repository

```bash
git clone https://github.com/itsmohitnarayan/ziphuff.git
cd ziphuff
```


### Build the Project

```bash
cargo build --release
```

### Run the Application

```bash
cargo run --release
```

### GUI Controls

1. **Input File**: Browse and select the input file.
2. **Output File**: Specify the output file path.
3. **Action**: Choose between `Compress` and `Extract`.
4. **Mode**: Choose between `Words` and `Chars`.
5. **Run**: Click to start the compression or extraction process.

## Example

- **Compress a file**:
  1. Select an input file.
  2. Specify the output file path.
  3. Choose `Compress` as the action.
  4. Select either `Words` or `Chars` mode.
  5. Click `Run`.

- **Extract a file**:
  1. Select a compressed input file.
  2. Specify the output file path.
  3. Choose `Extract` as the action.
  4. Select the mode that was used for compression (`Words` or `Chars`).
  5. Click `Run`.

## License

This project is licensed under the [GNU Affero General Public License v3.0](LICENSE).

## Contributing

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -am 'Add some feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Create a new Pull Request.

## Authors

- **Kumar Mohit** - *work* - [itsmohitnarayan](https://github.com/itsmohitnarayan)

## Mentor

- [Lakret](https://github.com/Lakret)

----------------

