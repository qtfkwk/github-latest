use anyhow::Result;
use clap::Parser;
use futures::{stream::FuturesOrdered, StreamExt};
use veg::Veg;

#[derive(Debug)]
struct Row {
    repository: String,
    tag: String,
}

impl Row {
    fn new(repository: &str, tag: &str) -> Box<Row> {
        Box::new(Row {
            repository: repository.to_string(),
            tag: tag.to_string(),
        })
    }
}

impl veg::Table for Row {
    fn row(&self) -> Vec<String> {
        vec![self.repository.clone(), self.tag.clone()]
    }
}

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
struct Cli {
    /// One or more GitHub repositories (`qtfkwk/github-latest`)
    repos: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let client = reqwest::Client::builder()
        .user_agent(env!("CARGO_PKG_NAME"))
        .build()?;

    let mut table = Veg::table("Repository|Latest\n-|-");

    let mut gets = cli
        .repos
        .iter()
        .map(|repo| client.get(format!("https://github.com/{repo}")).send())
        .collect::<FuturesOrdered<_>>();

    let mut i = 0;
    while let Some(res) = gets.next().await {
        let content = res?.text().await?;
        match content
            .lines()
            .filter(|line| line.contains("tag/"))
            .take(1)
            .next()
        {
            Some(line) => {
                let tag = line.split('"').skip(7).take(1).next().unwrap();
                let tag = tag.split('/').skip(5).take(1).next().unwrap();
                table.push(Row::new(&cli.repos[i], tag));
            }
            None => {
                table.push(Row::new(&cli.repos[i], "?"));
            }
        }

        i += 1;
    }

    println!("{}", table.markdown()?);

    Ok(())
}
