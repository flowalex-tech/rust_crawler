use std::io::Read;
use select::document::Document;
use select::predicate::Name;

fn main() {
    let client = reqwest::blocking::Client::new();
    let orgin_url = "https://docs.flowalex.tech/docs/";
    let mut  res = client.get(orgin_url).send().unwrap();
    println!("Status for {}: {}",orgin_url, res.status());

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    Document::from(body.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
}