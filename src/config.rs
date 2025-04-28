use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
pub enum CommitFormat {
    /// Common commit format (feat: description)
    Conventional,
    /// Bracketed format [feat] description
    Bracketed,
    /// Plain format description
    Plain,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub api_key: String,
    pub model: String,
    pub format: CommitFormat,
}

impl CommitFormat {
    pub fn get_prompt(&self) -> &'static str {
        match self {
            CommitFormat::Conventional => {
                "Please generate a concise and clear commit message using the common commit format (e.g. feat: add new feature)."
            }
            CommitFormat::Bracketed => {
                "Please generate a concise and clear commit message using the bracketed format (e.g. [feat] add new feature)."
            }
            CommitFormat::Plain => {
                "Please generate a concise and clear commit message using the plain format (e.g. add new feature)."
            }
        }
    }
}
