# signature-checker

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT/Apache 2.0](https://img.shields.io/badge/License-MIT%2FApache%202.0-blue.svg)](LICENSE)

A **Rust** and **Iced**-based **Windows file digital signature verification tool** with a graphical user interface (GUI). It verifies the digital signature status, signer information, and certificate chain validity of `.exe`, `.dll`, `.sys`, and other executable files.

---

## 📌 Features
- **Signature Verification**: Check if a file is signed, unsigned, or has an invalid signature.
- **Signer Information**: Extract details like signer name and certificate issuer.
- **Certificate Chain Validation**: Verify if certificates are expired or revoked.
- **Batch Processing**: Drag and drop files or folders for bulk verification.
- **Graphical Interface**: Built with [Iced](https://github.com/iced-rs/iced) for an intuitive user experience.
- **Cross-Platform Design**: Rust-based, with potential for future expansion beyond Windows.

---

## 📦 Installation

### Build from Source
1. **Install Rust**:
   Ensure you have [Rust](https://www.rust-lang.org/tools/install) and `cargo` installed:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the Repository**:
   ```sh
   git clone https://github.com/corerao/signature-checker.git
   cd signature-checker
   ```

3. **Build and Run**:
   ```sh
   cargo build --release
   cargo run --release
   ```
   The compiled binary will be available at `target/release/signature-checker.exe`.

---

## 🚀 Usage

### Graphical Interface
1. Launch the application.
2. Add files for verification via the **"Select File"** button or by **drag-and-drop**.
3. Click **"Verify"** to check the signature status.
4. Results will display:
   - File path
   - Signature status (Valid/Invalid/Unsigned)
   - Signer name
   - Certificate issuer
   - Certificate validity period

### Command-Line Mode (Optional)
If CLI support is added, run:
```sh
cargo run -- --help
```

---

## 🛠️ Dependencies
- [Iced](https://github.com/iced-rs/iced): GUI framework.


Add these to your `Cargo.toml`:
```toml
[dependencies]
iced = { version = "0.13.1", features = ["tokio"] }
tokio = { version = "1.0", features = ["full"] }
```

---

## 📂 Project Structure
```
signature-checker/
├── src/
│   ├── main.rs          # Entry point
│   ├── gui/             # Iced GUI logic
│   ├── core/            # Core signature verification logic
│   └── utils/           # Utility functions
├── Cargo.toml           # Rust configuration
└── README.md            # Project documentation
```

---

## 📸 Screenshot
(Add a screenshot or GIF demo here)

---

## 🤝 Contributing
Contributions are welcome! Submit **Issues** or **Pull Requests**. Ensure your code follows Rust best practices and passes `cargo test`.

---

## 📜 License
This project is licensed under **MIT** or **Apache 2.0**. See [LICENSE](LICENSE) for details.
