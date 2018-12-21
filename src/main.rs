use std::env;
extern crate rusutologs;
use reqwest::{Error, Response};
use std::fs;
use std::io;
use std::path::Path;
extern crate regex;
use regex::Regex;

fn help() {
    println!("Use:\nchan-down.exe <url> <args>");
    println!("Args:");
    println!("TODO");
}

fn download_thread(thread_link: &str, usename: bool) -> Result<(), Error> {
    rusutologs::info("Starting...");
    let thread_parse: Vec<&str> = thread_link.split('/').collect();

    let th_ps_th: Vec<&str> = thread_parse[5].split('#').collect();
    let board: &str = thread_parse[3];
    let mut thread: &str = th_ps_th[0];

    if thread_parse.len() > 6 {
        let th_ps_th_tmp: Vec<&str> = thread_parse[6].split('#').collect();
        let thread_tmp: &str = th_ps_th_tmp[0];

        if usename
            || Path::new(".")
                .join("downloads")
                .join(board)
                .join(thread_tmp)
                .exists()
        {
            thread = thread_tmp;
        }
    }

    if Path::new(".")
        .join("downloads")
        .join(board)
        .join(thread)
        .exists()
        != true
    {
        let thpa = format!("downloads/{}/{}/", board, thread);

        fs::create_dir_all(thpa);
    }

    let body = reqwest::get(thread_link)?.text()?;

    let regexx = Regex::new(r"(\x2f\x2fi\.4cdn\.org\x2f\w+\x2f\d+\.\w+)").unwrap();

    for link in regexx.captures_iter(&body) {
        let glink = format!("https:{}", &link[0]);
        let mut response = reqwest::get(&glink)?;
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        let fname2 = fname.clone();

        let fname = Path::new(".")
            .join("downloads")
            .join(board)
            .join(thread)
            .join(fname);
        let stringinfo = format!(
            "file to download: '{}' will be located under: {:?}",
            fname2, fname
        );
        rusutologs::info(&stringinfo);

        let mut out = fs::File::create(fname).expect("Fail");
        io::copy(&mut response, &mut out);
    }
    rusutologs::successful("Ok");
    Ok(())
}

fn main() {
    let mut usename: bool = false;
    let args: Vec<String> = env::args().collect();
    let mut url = "none";

    if args.len() == 1 {
        help();
    } else if args.len() >= 2 {
        url = &args[1];
        for x in args.clone() {
            if x == "-n" || x == "--use-names" {
                rusutologs::info("-n");
                usename = true;
            }
        }
    }

    if args.len() > 1 {
        download_thread(url, usename);
    }
}
