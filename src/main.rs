use scraper::Html;
use scraper::Selector;

fn main() {
    let html = r#"
    <html>
    <body>
    <div id="main"><h1>Content</h1></div>
    </body>
    </html>
    "#;

    let document = Html::parse_document(html);
    let selector = Selector::parse("#main").unwrap();

    let element = document.select(&selector).next().unwrap();

    println!("{:?}", element.inner_html());
}
