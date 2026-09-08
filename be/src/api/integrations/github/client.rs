use anyhow::{Result, anyhow};
use serde::Deserialize;

const GITHUB_API_BASE: &str = "https://api.github.com";
const API_VERSION: &str = "2022-11-28";

#[derive(Debug, Deserialize)]
pub struct GitHubUser {
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubIssue {
    pub number: i32,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
    /// Present (non-null) when the "issue" is actually a pull request —
    /// GitHub's issues endpoint returns both.
    pub pull_request: Option<serde_json::Value>,
}

fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("office-erp-github-connector")
        .build()
        .expect("failed to build reqwest client")
}

fn request(method: reqwest::Method, url: &str, token: &str) -> reqwest::RequestBuilder {
    client()
        .request(method, url)
        .bearer_auth(token)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", API_VERSION)
}

/// Confirms the PAT is valid and returns the authenticated GitHub user.
pub async fn validate_token(token: &str) -> Result<GitHubUser> {
    let url = format!("{GITHUB_API_BASE}/user");
    let resp = request(reqwest::Method::GET, &url, token)
        .send()
        .await
        .map_err(|e| anyhow!("Could not reach GitHub: {}", e))?;

    if !resp.status().is_success() {
        return Err(anyhow!("Invalid GitHub token"));
    }

    resp.json::<GitHubUser>()
        .await
        .map_err(|e| anyhow!("Unexpected response from GitHub: {}", e))
}

/// Confirms the connected account can read the given repo.
pub async fn validate_repo_access(token: &str, owner: &str, repo: &str) -> Result<()> {
    let url = format!("{GITHUB_API_BASE}/repos/{owner}/{repo}");
    let resp = request(reqwest::Method::GET, &url, token)
        .send()
        .await
        .map_err(|e| anyhow!("Could not reach GitHub: {}", e))?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Repository {owner}/{repo} is not accessible with the connected GitHub account"
        ));
    }

    Ok(())
}

/// Fetches every issue (open and closed, pull requests excluded) for a repo,
/// following pagination via the `Link` response header.
pub async fn fetch_issues(token: &str, owner: &str, repo: &str) -> Result<Vec<GitHubIssue>> {
    let mut all = Vec::new();
    let mut url = Some(format!(
        "{GITHUB_API_BASE}/repos/{owner}/{repo}/issues?state=all&per_page=100"
    ));

    while let Some(current_url) = url {
        let resp = request(reqwest::Method::GET, &current_url, token)
            .send()
            .await
            .map_err(|e| anyhow!("Could not reach GitHub: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            return Err(anyhow!(
                "GitHub API returned {} while fetching issues",
                status
            ));
        }

        let next_url = resp
            .headers()
            .get(reqwest::header::LINK)
            .and_then(|v| v.to_str().ok())
            .and_then(parse_next_link);

        let page: Vec<GitHubIssue> = resp
            .json()
            .await
            .map_err(|e| anyhow!("Unexpected response from GitHub: {}", e))?;

        all.extend(
            page.into_iter()
                .filter(|issue| issue.pull_request.is_none()),
        );
        url = next_url;
    }

    Ok(all)
}

fn parse_next_link(link_header: &str) -> Option<String> {
    for part in link_header.split(',') {
        let mut segments = part.split(';');
        let url_part = segments.next()?.trim();
        let is_next = segments.any(|s| s.trim() == r#"rel="next""#);
        if is_next {
            return Some(
                url_part
                    .trim_start_matches('<')
                    .trim_end_matches('>')
                    .to_string(),
            );
        }
    }
    None
}
