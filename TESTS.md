# Install Code Coverage Tools
```bash
cargo install cargo-llvm-cov
```

# Generate coverage report in terminal
```bash
cargo llvm-cov
```
# Generate HTML report
```bash
cargo llvm-cov --html
# Opens report at target/llvm-cov/html/index.html
```

# Generate lcov format (for CI tools)
```bash
cargo llvm-cov --lcov --output-path lcov.info
```

# Run specific tests
```bash
cargo llvm-cov --test test_name
```