// ## Задача
//
// Необходимо разработать CLI-утилиту совершающую HTTP Healthcheck'и по заданному URL'у.
//
// Проверка совершается в цикле с заданным интервалом. На каждой итерации необходимо совершить HTTP GET по заданному URL'у.
// Есть три возможных результата проверки:
// 1. `OK(200)`, в случае, если запрос вернул HTTP status code `200`.
// 2. `ERR({HTTP_CODE})`, в случае, если запрос вернул HTTP status code отличный от `200`.
// 3. `URL parsing error`, в случае, если второй аргумент не является валидным HTTP URL'ом. После чего приложение завершается.
//
// Утилита принимает два аргумента:
// 1. Целочисленное значение интервала в секундах.
// 2. HTTP URL который будет проверяться.

use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use ureq::Error;
use url::Url;

use test_task::Args;

fn main() {
    let args = Args::parse();

    loop {
        sleep(Duration::from_secs(args.interval as u64));
        let url = args.url.clone();
        println!("Check {}", args.url);
        validate(url.clone());
        match ureq::get(url.as_str()).call() {
            Ok(response) => {
                println!("Ok({})", response.status())
            }
            Err(Error::Status(code, _)) => {
                println!("Err({})", code)
            }
            Err(unexpected) => {
                println!("Unexpected error {}", unexpected.to_string())
            }
        }
    }
}

fn validate(url: String) {
    Url::parse(&*url).expect("URL parsing error");
}

#[test]
fn test() {}
