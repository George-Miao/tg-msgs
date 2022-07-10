use std::fs::File;

use telegram_chat::Analyzer;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let content = std::fs::read("data/result.json")?;
    let data = serde_json::from_slice(&content)?;
    let file = File::create("data/output.md")?;

    Analyzer::new(&data)
        .wrap_with("```")
        .take(15)
        .write_to(file)
        .opt_out(&[223683261])
        .count_link()?
        .sender_rank()?
        .count_substring("草")?;

    Ok(())
}
