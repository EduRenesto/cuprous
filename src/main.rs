#[macro_use]
extern crate json;

mod reddit;

fn main() {
    let brasil = reddit::subreddit::Subreddit {
        name: "brasil".to_string()
    };

    let links = brasil.get_links();
    for lnk in links.unwrap() {
        println!("{}: {} by {}", lnk.score, lnk.title, lnk.author);
    }
}
