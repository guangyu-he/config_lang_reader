# config_lang_serder

A Rust library and Python extension for reading and converting configuration files (TOML, YAML, JSON, XML) into Python
dictionaries. Powered by [PyO3](https://github.com/PyO3/pyo3), this project enables seamless integration of Rust's
performance with Python's usability for configuration management.

## Features

- **Read and parse TOML, YAML, JSON, and XML files**
- **Unified API:** Returns configuration as a Python `dict` regardless of format
- **High performance:** Leverages Rust for fast parsing
- **Python bindings:** Easily callable from Python via PyO3

## Installation

### by PyPI

```sh
uv pip install config-lang-reader
# or
pip install config-lang-reader
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

## Usage

### Python

```python
import config_lang_serder

# Automatically detects file type by extension
data = config_lang_serder.read('config.yaml')
print(data)
```

Current supported extensions: `.toml`, `.yaml`, `.json`, `.xml`

## API

- `read(path: str) -> dict`: Reads a config file and returns its contents as a Python dict. Supported extensions: toml,
  yaml, json, xml.
- `read_toml(path: str) -> dict`
- `read_yaml(path: str) -> dict`
- `read_json(path: str) -> dict`
- `read_xml(path: str) -> dict`

## Development

- Rust code in `src/`
- Python bindings via PyO3
- Tests in `test.py` and `tests/`

## License

MIT