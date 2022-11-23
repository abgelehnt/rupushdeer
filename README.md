# rupushdeer

[PushDeer](https://github.com/easychen/pushdeer) SDK for Rust.

## install

```toml
[dependencies]
rupushdeer = "0.1.0"
```

## Usage:

### 1. Use pushdeer default server

```rust
use rupushdeer::PushDeer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deer = PushDeer::new("PDU5315TCkMN0KiBRqbceaXxpYx3DvdbiZ3JpAIE");
    deer.send_text("title")?;
    deer.send_text_with_desp("Hello", "send_text_with_desp")?;
    deer.send_markdown("# markdown\npushdeer")?;
    deer.send_markdown_with_desp("# markdown\npushdeer", "send_markdown_with_desp")?;
    deer.send_image("https://gitee.com/easychen/pushdeer/raw/main/doc/image/clipcode.png")?;
    Ok(())
}
```

### 2. Use self-hosted server

```rust
use rupushdeer::PushDeer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deer = PushDeer::new("PDU5315TCkMN0KiBRqbceaXxpYx3DvdbiZ3JpAIE").set_server("http://127.0.0.1:12345");
    deer.send_text("title")?;
    Ok(())
}
```
