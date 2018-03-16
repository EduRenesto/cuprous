use reddit::link::*;
use json;

extern crate reqwest;


pub struct Subreddit {
    pub name: String
}

impl Subreddit {
    pub fn get_links(&self) -> reqwest::Result<Vec<Link>> {
        let mut url = "https://reddit.com/r/".to_owned();
        url.push_str(&self.name);
        url.push_str(".json");

        let content = reqwest::get(&url)?.text()?;
        let decoded = json::parse(&content).unwrap();

        let mut ret : Vec<Link> = Vec::new();

        for link in decoded["data"]["children"].members() {
            ret.push(link_from_json(link));
        }

        Ok(ret)
    }
}
