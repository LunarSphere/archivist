/* 
* I want to build a web crawler in rust, 
* dependenceies I'll need tokio, reqwest, scraper, url 
* web crawler process
* start with seed url, fetch page, parse html, extract links, and content, add new urls to queue, repeat
* respect robots.txt, we are crawling and scraping, crawliing is IO-bound so we need tokio for concurrency 
* 
 */
// use thiserror::Error;
use select::document::Document;
use select::predicate::Name;
use url::Url;
use std::collections::VecDeque;




//TODO: add error handling, respect robots.txt~, add delay between requests, limit crawl depth, handle dynamic content, add user-agent header, implement concurrency with tokio tasks. 
//TODO: avoid revisiting urls 

// collect urls can fail so we must update the function signature to return an error
async fn collecturls(seed: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let base_url = Url::parse(seed)?;

    //grab body of the urlpage
    let body = reqwest::get(base_url.as_str())
    .await? //wait for the page to respond
    .text() //process information as text. 
    .await?;

    // list of links
    let links: Vec<String> = Document::from(body.as_str())
      .find(Name("a"))
      .filter_map(|node| node.attr("href"))
      .filter_map(|href| {
        if href.starts_with('#') || href.is_empty(){
            return None;
        }
        base_url.join(href).ok().map(|url| url.to_string())
      })
      .collect();

      return Ok(links);
}

//url lets us add relative urls to base urls
#[tokio::main] //designates main as an async function 
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut queue = VecDeque::new();
    queue.push_back("https://books.toscrape.com/".to_string());

    
    while let Some(links) = queue.pop_front(){
        let new_links = collecturls(&links).await?;
        for link in new_links{
            println!("{}", link);
            queue.push_back(link)
        }
    }

    Ok(()) //this line means we executed the code without any errors
}


//some pages are dynamic. the load their content with JS after page loads. 
// if we fetch a site like this with request we'll get bare htlm. because reqwest only makes html request
// to load the bage we need the API, a headless browserlike (selinium), scraping service. 

//4/19 
// implemented a basic webcrawler in rust