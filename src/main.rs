#[macro_use]
extern crate prettytable;

use chrono::Duration;
use clap::Parser;
mod client;
use prettytable::{Cell, Row, Table};
use reqwest::Error;
use std::result::Result;

mod api;
mod model;

#[derive(Parser, Debug)]
#[clap(version)]
#[clap(name = "Development Analyzer")]
#[clap(author = "Joan Flotats")]
#[clap(version = "0.1.0")]
#[clap(about = "Analyze Development Cycle Time", long_about = None)]
struct Args {
    /// Repository owner
    #[clap(short, long)]
    owner: String,

    /// Repository
    #[clap(short, long)]
    repo: String,

    /// GitHub Token
    #[clap(short, long)]
    token: String,

    /// Start Date
    #[clap(long)]
    from_date: Option<String>,

    /// Finish Date
    #[clap(long)]
    to_date: Option<String>,

    /// Branch Name
    #[clap(short, long, default_value_t = String::from("main"))]
    branch: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let client = client::GithubClient::new(args.token);

    let pull_requests = client
        .pulls()
        .list(args.owner, args.repo, args.branch)
        .await?;

    let mut lead_time = Duration::seconds(0);

    for pull_request in pull_requests.iter() {
        lead_time = lead_time + pull_request.lead_time();
    }

    let avg_lead_time = Duration::minutes(lead_time.num_minutes() / pull_requests.len() as i64);

    let mut table = Table::new();

    table.add_row(row!["PR", "PR Lead Time"]);

    for pull_request in pull_requests.iter() {
        table.add_row(row![
            pull_request.number,
            format!("{}", pretty_duration(pull_request.lead_time()))
        ]);
    }

    table.add_row(row![
        format!("COUNT: {}", pull_requests.len()),
        format!("AVG: {}", pretty_duration(avg_lead_time))
    ]);

    table.printstd();

    Ok(())
}

fn pretty_duration(duration: Duration) -> String {
    let m = duration.num_seconds() / 60;

    let h = m / 60;
    let minutes = m % 60;

    let days = h / 24;
    let hours = h % 24;

    format!("{}D {}H {}M", days, hours, minutes)
}
