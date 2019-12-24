use reqwest::header::USER_AGENT;
use std::env;
use std::process::exit;
use reqwest::Url;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() !=2 {
        println!("Usage: {} <adfly-link>", args[0]);
        exit(1)
    }
    let url: Url = args[1].to_string().parse().unwrap();

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36")
        .send()
        .unwrap()
        .text()
        .unwrap();

    let ysmm: &str = res
        .split("ysmm = '")
        .nth(1)
        .unwrap()
        .split("';")
        .nth(0)
        .unwrap();
    let decrypt = decrypt(ysmm);
    println!("{}", decrypt);
}

fn decrypt(ysmm: &str) -> String {
    let mut zeros: String = "".to_string();
    let mut ones: String = "".to_string();
    for (num, letter) in ysmm.chars().enumerate() {
        if num % 2 == 0 {
            zeros.push(letter);
        } else {
            let scrr = ones;
            ones = "".to_string();
            ones.push(letter);
            ones.push_str(scrr.as_ref());
        }
    }
    let mut response = zeros;
    response.push_str(ones.as_str());
    let mut chararray: Vec<char> = response.chars().collect();
    let mut i = 0;

    while i < chararray.len() {
        if chararray[i].is_digit(10) {
            let mut j = i + 1;
            while j < chararray.len() {
                if chararray[j].is_digit(10) {
                    let s = chararray[i] as u8 ^ chararray[j] as u8;
                    if s < 7 {
                        chararray[i] = (s as u8 + 48) as char;
                    }
                    i = j;
                    j = chararray.len();
                }
                j += 1;
            }
        }
        i += 1;
    }

    response = chararray.into_iter().collect();
    let key = base64::decode(&response).unwrap();
    response = String::from_utf8_lossy(key.as_ref()).to_string();
    let response = &response[16..];
    (&response[..response.len() - 16].to_owned()).parse().unwrap()
}
