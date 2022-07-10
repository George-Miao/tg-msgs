use std::fs::File;

use telegram_chat::Analyzer;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;

    let content = std::fs::read("data/result.json")?;
    let data = serde_json::from_slice(&content)?;
    let file = File::create("data/output.md")?;
    let opt_out = std::env::var("OPT_OUT")
        .map(|x| {
            x.split(',')
                .map(|x| {
                    x.trim()
                        .parse::<i64>()
                        .expect("invalid opt_out: requires `i64`")
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    Analyzer::new(&data)
        .wrap_with("```")
        .take(15)
        .write_to(file)
        .opt_out(&opt_out)
        .count_link()?
        .sender_rank()?
        .count_substring("草")?;

    Ok(())
}
