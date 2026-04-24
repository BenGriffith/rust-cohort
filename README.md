# rust-json-parser

A high-performance JSON library built from scratch in **Rust** (2024 Edition) and exposed as a native **Python** extension via **PyO3** and **Maturin**. This project features a hand-rolled recursive descent parser and a custom tokenizer for robust lexical analysis.

---

## 🚀 Features

* **Recursive Descent Parser:** Hand-rolled logic for converting strings into a type-safe Abstract Syntax Tree (AST).
* **Custom Tokenizer:** Efficiently handles multi-byte characters and complex escape sequences.
* **Python Bindings:** Seamlessly integrated with Python 3.12+ for high-speed serialization and deserialization.
* **CLI Interface:** Built-in support for parsing and pretty-printing JSON directly from the terminal or via pipes.
* **Performance Benchmarking:** Includes internal tools (`benchmark_performance`) to measure parsing speeds against standard libraries.

---

## 🛠️ Project Structure

The project uses a hybrid layout to manage the Rust crate and the Python package concurrently:

```text
.
├── python/
│   └── rust_json_parser/
│       ├── __init__.py          # Public Python API exports
│       └── __main__.py          # CLI entry point
├── src/
│   ├── error.rs                 # Custom error types
│   ├── lib.rs                   # Crate root
│   ├── main.rs                  # Rust binary entry point
│   ├── parser.rs                # Core parsing logic
│   ├── python_bindings.rs       # PyO3 bridge code
│   ├── tokenizer.rs             # Lexical analysis
│   └── value.rs                 # JsonValue AST & formatting logic
├── test_data/                   # Sample JSON files for tests/benchmarks
├── tests/                       # Integration tests
├── .gitignore
├── Cargo.lock
├── Cargo.toml                   # Rust metadata & dependencies
└── pyproject.toml               # Python build system & CLI configuration
```

---

## 💻 Installation

To build the extension for your local Python environment, ensure you have the Rust toolchain and `maturin` installed:

```bash
pip install maturin
maturin develop
```

---

## 📖 Usage

### Python API

The library provides a clean interface for typical JSON operations:

```python
import rust_json_parser as rjp

# Parse JSON string
result = rjp.parse_json('{"name": "Tim", "age": 30}')
print(result["name"])  # "Tim"

# Parse from file
data = rjp.parse_json_file('config.json')

# Serialize back to JSON
json_str = rjp.dumps({"key": "value"}, indent=2)
```

### Command Line Interface

Your package can also be run as a command-line tool:

```bash
# Parse a JSON file
python3 -m rust_json_parser data.json

# Parse an inline JSON string
python3 -m rust_json_parser '{"hello":"world"}'

# Parse from stdin (pipe)
echo '{"test": 123}' | python3 -m rust_json_parser
```

---

## 🧪 Testing & Validation

The project includes an extensive test suite validating core parser logic and FFI boundaries.

```bash
# Run standard unit tests
cargo test

# Run documentation tests
cargo test --doc
```

---

## ⚙️ Implementation Details

* **Tokenization:** Input is processed to ensure correct UTF-8 boundary handling during lexical analysis.
* **Error Handling:** Provides specific feedback on malformed JSON, including the exact position of unexpected tokens.
* **Formatting:** The `pretty_print` implementation uses a recursive strategy to allow customizable indentation levels for generated strings.

