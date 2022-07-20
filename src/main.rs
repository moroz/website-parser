use glob::glob;
use scraper::Html;
use scraper::Selector;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn parse_file(path: &PathBuf) -> io::Result<scraper::Html> {
    let html = fs::read_to_string(&path)?;
    let dom = Html::parse_document(&html);
    Ok(dom)
}

fn extract_main(dom: &scraper::Html) -> Option<String> {
    let main_selector = Selector::parse("#main").unwrap();
    dom.select(&main_selector)
        .next()
        .map_or(None, |elem| Some(elem.inner_html().trim().to_string()))
}

fn delete_viagra_ad(html: &str) -> String {
    let mut lines: Vec<_> = html.lines().collect();
    lines.remove(lines.len() - 1);
    lines.join("\n")
}

fn main() -> io::Result<()> {
    let path = env::current_dir().unwrap();
    let outpath = &path.parent().unwrap().join("out");
    if !outpath.is_dir() {
        fs::create_dir(&outpath)?;
    }
    let files_path = path
        .parent()
        .unwrap()
        .join("www.bartolty.buddyzm.pl/**/*.html");
    let files_path = files_path.to_str().unwrap();
    let paths = glob(&files_path).unwrap();
    for path in paths {
        let path = path.unwrap();
        let parsed = parse_file(&path)?;
        if let Some(main) = extract_main(&parsed) {
            let filename = path.file_name().unwrap();
            let target = &outpath.join(filename);
            let main = delete_viagra_ad(&main);
            fs::write(target, main)?;
        }
    }

    Ok(())
}
