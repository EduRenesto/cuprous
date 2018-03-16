extern crate json;

pub struct Link {
    pub id: String,
    pub title: String,
    pub author: String,
    pub score: i32
}

pub fn link_from_json(val: &json::JsonValue) -> Link {
    Link {
        id: val["data"]["id"].to_string(),
        title: val["data"]["title"].to_string(),
        author: val["data"]["author"].to_string(),
        score: val["data"]["score"].to_string().parse::<i32>().unwrap()
    }
}
