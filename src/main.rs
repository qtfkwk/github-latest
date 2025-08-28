use anyhow::Result;
use clap::Parser;
use futures::{StreamExt, stream::FuturesOrdered};
use std::collections::HashSet;
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
    /// Exclude tags with
    #[arg(short, default_value = "rc,pre,canary")]
    exclude: String,

    /// Show all tags (on the first tags page)
    #[arg(short)]
    all: bool,

    /// Quiet mode; just show the latest tag(s)
    #[arg(short)]
    quiet: bool,

    /// One or more GitHub repositories (`qtfkwk/github-latest`)
    #[arg(value_name = "REPO")]
    repos: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let exclude = if cli.exclude.is_empty() {
        HashSet::new()
    } else {
        cli.exclude.split(',').collect::<HashSet<_>>()
    };

    let client = reqwest::Client::builder()
        .user_agent(env!("CARGO_PKG_NAME"))
        .build()?;

    let mut table = Veg::table("Repository|Latest\n-|-");

    let mut gets = cli
        .repos
        .iter()
        .map(|repo| client.get(format!("https://github.com/{repo}/tags")).send())
        .collect::<FuturesOrdered<_>>();

    let mut i = 0;
    while let Some(res) = gets.next().await {
        let content = res?.text().await?;

        let tags = content
            .lines()
            .filter(|line| line.contains("tag/"))
            .filter_map(|line| {
                if let Some(tag) = line.split('"').skip(5).take(1).next() {
                    tag.split('/').skip(5).take(1).next()
                } else {
                    None
                }
            })
            .map(|tag| urlencoding::decode(tag).unwrap())
            .filter(|tag| exclude.iter().all(|x| !tag.contains(x)))
            .collect::<Vec<_>>();

        let t = if tags.is_empty() {
            "?"
        } else if cli.all {
            &tags.join(", ")
        } else {
            &tags.first().unwrap().to_string()
        };

        if cli.quiet {
            println!("{t}");
        } else {
            table.push(Row::new(&cli.repos[i], t));
        }

        i += 1;
    }

    if !cli.quiet {
        println!("{}", table.markdown()?);
    }

    Ok(())
}
