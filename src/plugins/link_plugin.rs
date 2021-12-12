use irc::client::prelude::*;
use regex::Regex;
use scraper::{Html, Selector};

static PATTERN: &str = r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&/=]*)";

pub struct LinkPlugin {
    re: Regex,
}

impl LinkPlugin {
    pub fn new() -> LinkPlugin {
        LinkPlugin {
            re: Regex::new(PATTERN).unwrap(),
        }
    }
}

#[async_trait]
impl super::Plugin for LinkPlugin {
    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            self.re.is_match(&msg)
        } else {
            false
        }
    }

    async fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()> {
        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            for cap in self.re.captures_iter(msg) {
                match reqwest::get(&cap[0]).await {
                    Ok(resp) => match resp.text().await {
                        Ok(text) => match parse_title(&text) {
                            Some(title) => {
                                client.send_privmsg(target, format!("link: {}", title))?
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

fn parse_title(text: &str) -> Option<String> {
    let fragment = Html::parse_document(text);
    let title_selector = Selector::parse("title").unwrap();

    fragment
        .select(&title_selector)
        .next()
        .map(|n| n.text().collect())
}
