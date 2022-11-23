use rupushdeer::PushDeer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let deer = PushDeer::new("PDU5315TCkMN0KiBRqbceaXxpYx3DvdbiZ3JpAIE");
    deer.send_text("title")?;
    deer.send_text_with_desp("Hello", "send_text_with_desp")?;
    deer.send_markdown("# markdown\npushdeer")?;
    deer.send_markdown_with_desp("# markdown\npushdeer", "send_markdown_with_desp")?;
    deer.send_image("https://gitee.com/easychen/pushdeer/raw/main/doc/image/clipcode.png")?;
    Ok(())
}
