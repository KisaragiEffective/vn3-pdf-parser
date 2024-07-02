use std::path::PathBuf;

use clap::Parser;

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
