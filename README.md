# Rusty Parser

**Rusty Parser** is an XML-like parser built in Rust, inspired by the guide at [bodil.lol/parser-combinators](https://bodil.lol/parser-combinators/).  
This parser takes input in an XML-like format and converts it into a CLI command.  

### Example input  
```xml
<vim/> <text.txt/> </text.txt>
```

## Requirements

- **Rust toolchain installed**. You can install it from https://rustup.rs/.

## Usage

```bash
git clone https://github.com/anas-azouane/RustyParser.git  
cd rusty_parser  
cargo build  
./target/debug/rusty_parser input
```

