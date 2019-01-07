use std::env;
use std::fs;
use std::io;
use std::path::Path;
use reqwest::{Error, Response};
extern crate regex;
use regex::Regex;
extern crate rusutologs;
extern crate clap;
use clap::{Arg, App, SubCommand};

fn download_thread(thread_link: &str, chan: &str) -> Result<(), Error> {
    rusutologs::info("Starting...");
    if thread_link == "none" {
        rusutologs::error("URL is missing.");
        std::process::exit(1)
    }

    let thread_parse: Vec<&str> = thread_link.split('/').collect();
    let domain = thread_parse[2].clone();
    let th_ps_th: Vec<&str> = thread_parse[5].split('#').collect();
    let board: &str = thread_parse[3];
    let mut thread: &str = th_ps_th[0];

    if thread_parse.len() > 6 {
        let th_ps_th_tmp: Vec<&str> = thread_parse[6].split('#').collect();
        let thread_tmp: &str = th_ps_th_tmp[0];
    }

    if Path::new(".")
        .join("downloads")
        .join(chan)
        .join(board)
        .join(thread)
        .exists()
        != true
    {
        let thpa = format!("downloads/{}/{}/{}/", chan, board, thread);

        fs::create_dir_all(thpa);
    }

    let body = reqwest::get(thread_link)?.text()?;
    let mut regexx = Regex::new(r"(\x2f\x2flolnada\.org\x2f\w+\x2fsrc\x2flolnada\.org-\d+\.\w+)").unwrap();
    //Funcionan: 4chan, 2chan, lolnada, wizchan, hispachan; demás sin testear.
    match chan {
        "4chan" => {regexx = Regex::new(r"(\x2f\x2fi\.4cdn\.org\x2f\w+\x2f\d+\.\w+)").unwrap();}
        "2chan" => {regexx = Regex::new(r"(\x2f\w+\x2fsrc\x2f\d+\.\w+)").unwrap();}
        "8chan" => {regexx = Regex::new(r"(\x2f\x2f\w+\.8ch\.net\x2ffile_store\x2f\w+\.\w+)").unwrap();}
        "420chan" => {regexx = Regex::new(r"(\x2f\w+\x2fsrc\x2f\d+\.\w+)").unwrap();}
        "wizchan" => {regexx = Regex::new(r"(\x2f\w+\x2fsrc\x2f\d+\.\w+)").unwrap();}
        "lainchan" => {regexx = Regex::new(r"(\x2f\w+\x2fsrc\x2f\d+\.\w+)").unwrap();}
        "hispachan" => {regexx = Regex::new(r"(\x2f\x2f\w+\.hispachan\.org\x2f\w+\x2fsrc\x2f\d+\.\w+)").unwrap();}
        "lolnada" => {regexx = Regex::new(r"(\x2f\x2flolnada\.org\x2f\w+\x2fsrc\x2flolnada\.org-\d+\.\w+)").unwrap();}
        _ => {rusutologs::error("Bad chan");}
    }

    for link in regexx.captures_iter(&body) {
        let mut glink = format!("https:{}", &link[0]);
        //Comprueba si el dominio es falto en el documento para aquellos que lo requieran.
        match chan {
            "4chan" => {glink = format!("https:{}", &link[0]);}
            "2chan" => {glink = format!("https://{}{}", domain, &link[0]);}
            "420chan" => {glink = format!("https://{}{}", domain, &link[0]);}
            "wizchan" => {glink = format!("https://{}{}", domain, &link[0]);}
            "lainchan" => {glink = format!("https://{}{}", domain, &link[0]);}
            _ => {glink = format!("https:{}", &link[0]);}
        }

        let mut response = reqwest::get(&glink)?;
        //let mut response = reqwest::get(&link[0])?;
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        let fname2 = Path::new(".")
            .join("downloads")
            .join(chan)
            .join(board)
            .join(thread)
            .join(fname);
        let stringinfo = format!(
            "file to download: '{}' will be located under: {:?}",
            fname, fname2
        );
        rusutologs::info(&stringinfo);

        let mut out = fs::File::create(fname2).expect("Fail");
        io::copy(&mut response, &mut out);
    }
    rusutologs::successful("Ok");
    Ok(())
}

fn down() {
    
}

fn main() {
    let matches = App::new("Chan-down")
        .version("1.0")
        .about("A 4chan downloader in rust.\nCurrent support: 2chan, 4chan, 8chan, 420chan, Wizchan, Lainchan, Hispachan, Lolnada")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .value_name("URL")
            .help("Url to work with")
            .takes_value(true))
        .arg(Arg::with_name("service")
            .short("c")
            .long("chan")
            .value_name("CHAN")
            .help("Service to work with (default: 4chan)")
            .takes_value(true))
        .get_matches();
    download_thread(matches.value_of("url").unwrap_or("none"), matches.value_of("service").unwrap_or("4chan"));
}