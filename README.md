# config_lang_serder

A Rust library and Python extension for reading and converting configuration files (TOML, YAML, JSON, XML) into Python
dictionaries and vice versa. Powered by [PyO3](https://github.com/PyO3/pyo3), this project enables seamless integration
of Rust's
performance with Python's usability for configuration management.

## Features

- **Read and parse TOML, YAML, JSON, and XML (not fully supported yet) files**
- **Unified API:** Returns configuration as a Python `dict` regardless of format
- **High performance:** Leverages Rust for fast parsing
- **Python bindings:** Easily callable from Python via PyO3
- **Write Python dict to TOML, YAML, or JSON file**

## Installation

### by PyPI

```sh
uv pip install config-lang-serder
# or
pip install config-lang-serder
```

### from source (via maturin or setuptools-rust)

Build and install with [maturin](https://github.com/PyO3/maturin):

```sh
maturin develop
```

Or build a wheel:

```sh
maturin build
pip install target/wheels/config_lang_serder-*.whl
```

### Requirements

- Python >= 3.10
- Rust toolchain (for building)

## Usage

```python
import config_lang_serder

# Read: Automatically detects file type by extension
config = config_lang_serder.read('config.yaml')
print(config)

# Write: Automatically detects file type by extension
config_lang_serder.write({'foo': 123, 'bar': True}, 'config.yaml')
```

Current supported extensions: `.toml`, `.yaml`, `.json`, `.xml` (read), `.toml`, `.yaml`, `.json` (write)

## API

- `read(path: str) -> dict`: Reads a config file and returns its contents as a Python dict. Supported extensions: toml,
  yaml, json, xml.
- `write(dict: dict, path: str) -> None`: Writes a Python dict to a file. The format is determined by the file
  extension (supports: toml, yaml, json).

## Development

- Rust code in `src/`
- Python bindings via PyO3
- Tests in `test.py` and `tests/`

## License

MIT