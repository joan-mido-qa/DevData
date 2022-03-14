use reqwest::{Client, Error};

use crate::{client::GithubClient, model::pulls::PullRequest};

pub struct Pulls<'client> {
    client: &'client Client,
}

impl<'client> Pulls<'client> {
    pub fn new(gh_client: &'client GithubClient) -> Self {
        Self {
            client: &gh_client.client,
        }
    }

    pub async fn list(
        &self,
        owner: String,
        repo: String,
        base: String,
    ) -> Result<Vec<PullRequest>, Error> {
        let mut pulls_list: Vec<PullRequest> = Vec::new();
        let mut pulls_page: Vec<PullRequest> = Vec::new();

        let mut page: u32 = 0;

        loop {
            let res = self
                .client
                .get(format!(
                    "https://api.github.com/repos/{owner}/{repo}/pulls?state=closed&page={page}&per_page=100&base={base}",
                    owner = owner,
                    repo = repo,
                    page = page,
                    base = base,
                ))
                .send()
                .await?;

            pulls_page = res.json::<Vec<PullRequest>>().await?;

            if pulls_page.len() == 0 {
                break;
            }

            pulls_list.append(&mut pulls_page);

            page += 1;
        }

        Ok(pulls_list)
    }
}
