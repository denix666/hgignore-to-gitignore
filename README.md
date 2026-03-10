# hgignore-to-gitignore

A fast and reliable command-line tool to convert Mercurial (Hg) `.hgignore` files to Git `.gitignore` format.

## Features

- ✅ Automatic detection of `.hgignore` files
- ✅ Converts glob patterns from hgignore to gitignore format
- ✅ Handles regexp syntax conversions
- ✅ Preserves comments and empty lines
- ✅ Optional removal of original `.hgignore` file
- ✅ Comprehensive error messages
- ✅ No external dependencies (minimal, pure Rust)
- ✅ Statically compiled with musl support
- ✅ Zero warnings, fully tested

## Installation

### From Source

Requirements:
- Rust 1.70 or later

```bash
git clone https://github.com/denix666/hgignore-to-gitignore.git
cd hgignore-to-gitignore
cargo build --release
```

The compiled binary will be at `target/release/hgignore-to-gitignore`.

### Precompiled Binaries

You can download precompiled binaries for Linux (x86_64 with musl) from the [releases page](https://github.com/denix666/hgignore-to-gitignore/releases).

## Usage

### Basic Usage

Simply run the tool in a directory containing a `.hgignore` file:

```bash
hgignore-to-gitignore
```

This will:
1. Check if `.hgignore` exists in the current directory
2. Convert the patterns to gitignore format
3. Write the result to `.gitignore`
4. Keep the original `.hgignore` file intact

### Remove Original File

To replace the `.hgignore` file with `.gitignore` after conversion:

```bash
hgignore-to-gitignore --replace_and_remove
# or use the short form
hgignore-to-gitignore -r
```

### Help

```bash
hgignore-to-gitignore --help
```

## Supported Patterns

### Glob Patterns (default)

The tool supports standard glob patterns used in `.hgignore`:

```
# Ignore all Python bytecode files
*.pyc
*.pyo

# Ignore build directories
build/
dist/

# Ignore specific files
.DS_Store
.env

# Wildcards
src/**/*.o
*.tmp?

# Character classes
file[0-9].txt
```

### Syntax Directives

The tool handles syntax declarations in `.hgignore`:

```
syntax: glob
*.pyc
*.pyo

syntax: regexp
^build/.*
^\..*
```

Supported syntaxes:
- `glob` - Standard glob patterns (default)
- `regexp` - Regular expressions
- `relre` - Relative regular expressions

### Inline Pattern Prefixes

- `glob:pattern` - Explicitly marks a glob pattern
- `re:pattern` - Explicitly marks a regex pattern

## Examples

### Example 1: Simple Python Project

**Input (.hgignore):**
```
syntax: glob

# Python
*.pyc
*.pyo
__pycache__/
*.egg-info/
dist/
build/

# IDE
.vscode/
.idea/
*.swp

# OS
.DS_Store
```

**Output (.gitignore):**
```
# Python
*.pyc
*.pyo
__pycache__/
*.egg-info/
dist/
build/

# IDE
.vscode/
.idea/
*.swp

# OS
.DS_Store
```

### Example 2: Converting with Regex

**Input (.hgignore):**
```
syntax: regexp
^\..*
^build/.*
.*\.o$
```

**Output (.gitignore):**
```
\..*
build/.*
.*\.o
```

## Error Handling

If `.hgignore` is not found in the current directory, the tool will display:

```
Error: File '.hgignore' not found in current directory.
Please ensure you run this command from the directory containing the .hgignore file.
```

If there are permission issues reading or writing files:

```
Error: Failed to write .gitignore: Permission denied
```

## Compilation Details

### Release Build

The project is optimized for performance:

```bash
cargo build --release
```

**Optimizations enabled:**
- `opt-level = 3` - Maximum optimization
- `lto = true` - Link-time optimization
- `codegen-units = 1` - Single codegen unit for better optimization
- `strip = true` - Strip binary symbols

### Musl Static Linking

To compile with musl for completely static binaries:

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

This produces a fully static binary with no glibc dependencies, perfect for Docker containers and Alpine Linux.

### Testing

Run the comprehensive test suite:

```bash
cargo test
```

All tests pass with no warnings or errors.

## Differences Between hgignore and gitignore

While both formats support glob patterns, there are some differences handled by this tool:

| Feature | hgignore | gitignore | Handled By Tool |
|---------|----------|-----------|-----------------|
| Glob patterns | ✅ | ✅ | Direct copy |
| Syntax directive | ✅ | ❌ | Removed |
| Comments | ✅ | ✅ | Preserved |
| Regexp support | ✅ | ⚠️ Limited | Converted where possible |
| Directory-only patterns | ✅ (ending with `/`) | ✅ | Preserved |
| Negation patterns | ✅ (`!`) | ✅ | Preserved |


## Performance

The tool is extremely fast due to:
- No regex compilation unless needed
- Efficient string operations
- Minimal allocations
- Single-pass processing

Typical conversion time: < 1ms for standard ignore files

## License

This project is licensed under the MIT License. See the LICENSE file for details.
