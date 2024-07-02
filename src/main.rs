use std::path::PathBuf;

use clap::Parser;

#[cfg(target_pointer_width = "32")]
compile_error!("This product and target may be vulnerable to RUSTSEC-2022-0041: please see https://github.com/crossbeam-rs/crossbeam/pull/781");

#[derive(Parser)]
struct Args {
    path: PathBuf
}

fn main() {
    let args = Args::parse();

    let file = pdf::file::FileOptions::cached().open(args.path).expect("can't read PDF");
    let resolver = file.resolver();
    let page =  file.pages().next().expect("there's no page").expect("there's no page");
    let flow = pdf_text::run(&file, &page, &resolver).expect("can't read PDF text");

    for line in flow.lines {
        println!("line: {line:?}");
    }

    for run in flow.runs {
        println!("run: {run:?}");
    }
    // println!("{pdf:?}");
}
