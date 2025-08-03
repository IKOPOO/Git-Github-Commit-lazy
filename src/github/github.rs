use anyhow::Ok;
use octocrab::Octocrab;

#[derive(Debug)]
pub struct GithubClient {
  octocrab: Octocrab,
}

impl GithubClient {
  pub async fn new(token: String) -> Result<Self, anyhow::Error> {
    let octocrab = Octocrab::builder().personal_token(token).build()?;
    Ok(Self { octocrab })
  }

  pub fn client(self) -> Octocrab {
    self.octocrab
  }
}
