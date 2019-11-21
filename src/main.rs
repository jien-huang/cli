use std::io::{stdout, Write};

use curl::easy::Easy;

// Print a web page onto stdout
fn main() {
    let mut easy = Easy::new();
    get_url(&mut easy, "https://www.rust-lang.org/");

    println!("{}", easy.response_code().unwrap());
}

fn get_url(easy: &mut Easy, url: &str) {
    easy.url(url).unwrap();
    easy.write_function(|data| {
//        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_url() {
        let mut easy = Easy::new();
        get_url(&mut easy, "https://www.google.com/");
        assert!(easy.response_code().unwrap() > 0);
    }

}