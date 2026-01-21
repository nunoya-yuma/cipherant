# README

## Environment

### Ollama install

```shell
curl -fsSL https://ollama.com/install.sh | sh
```

## Usage

### Build and execution

```shell
cargo run -- ${url}
# e.g.)
cargo run -- https://example.com
```

### Format

```shell
cargo fmt
```

### Lint

```shell
cargo clippy
```

## Test

### Unit test

```shell
cargo test
```

### E2E test

```shell
cargo test -- --ignored
```
