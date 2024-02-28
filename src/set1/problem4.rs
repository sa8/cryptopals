use crate::set1::problem3;

pub fn get_file(url: &str){
    let response = reqwest::blocking::get(url)
    .unwrap()
    .text()
    .unwrap();
   // println!("{}",response);
    //let document = scraper::Html::parse_document(&response);
    // Iterate over lines in the string
    for line in response.lines() {
        let decipher = problem3::bruteforce(line);
        println!("{}", decipher);
        
    }
}