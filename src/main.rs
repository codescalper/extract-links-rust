use std::io;

use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! { // TO handle std and reqwest error
    foreign_links{
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Enter the url of the website: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read ");
    let res = reqwest::get(input).await?.text().await?;
    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href")) // * used closure . it's used to process individual anchor elements
        .for_each(|x| println!("{}", x)); //*|x| is used to process the values of the "href" attribute extracted from those anchor elements. and pints value to the console */
    Ok(())
}
