use std::env;
use std::io::{stdout, Write};
use std::process::Command;
use std::process::exit;
use curl::easy::Easy;

pub fn get_url(easy: &mut Easy, url: &str) {
    //    let mut easy = Easy::new();
//    get_url(&mut easy, "https://www.rust-lang.org/");
//
//    println!("{}", easy.response_code().unwrap());
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