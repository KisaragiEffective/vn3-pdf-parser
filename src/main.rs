use std::error::Error;
use std::path::PathBuf;

use clap::Parser;
use ordered_float::NotNan;
use pdf_text::entry::Flow;

use crate::model::Vn3License;

mod model;

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

    let export = extract(flow).expect("failed to extract");

    println!("{export:?}");
    // println!("{pdf:?}");
}

fn extract(flow: Flow) -> Result<Vn3License, Box<dyn Error>> {
    const MARKERS: [&str; 9] = [
        "1.利用主体",
        "2.オンラインサービスへのアップロード",
        "3.センシティブな表現",
        "4.加工",
        "5.再配布・配布",
        "6.メディア・プロダクトへの利用",
        "7.二次創作",
        "8.その他",
        "9.特記事項"
    ];

    // TODO: Vec<Option<T>>は扱いにくいのでPartialMoveableVec<T>みたいな構造体で包む
    let mut words = flow.runs.into_iter()
        .flat_map(|x| x.lines)
        .chain(flow.lines)
        .flat_map(|line| line.words)
        // ゴミデータがはさまる
        .filter(|w| !w.text.is_empty())
        //
        .map(|w| Some(w))
        .collect::<Vec<_>>();

    // 前提: 上から下、左から右 (後者は自動的に満たされるものとする)
    words.sort_by_key(|w| NotNan::new(w.as_ref().unwrap().rect.y).unwrap());

    for word_ref in &words {
        let is_marker = MARKERS.contains(&word_ref.as_ref().unwrap().text.as_str());
        println!("{word_ref:?} ({is_marker})");
    }

    let mut extract_in_aabb0 = |x_lower_inclusive: f32, x_upper_inclusive: f32, y_lower_inclusive: f32, y_upper_inclusive: f32| -> String {
        words.iter_mut()
            .filter(|x| x.is_some())
            .filter(|word| {
                let target = word.as_ref().unwrap().rect;
                // TODO: Range<f32>
                target.x >= x_lower_inclusive && target.x <= x_upper_inclusive && target.y >= y_lower_inclusive && target.y <= y_upper_inclusive
            })
            .map(|x| x.take().expect("referencing already consumed snippet"))
            // 出現順序を保ちながら連結される
            .map(|x| x.text)
            .collect::<String>()
    };

    let mut extract_in_aabb = |x_lower_inclusive: f32, x_upper_inclusive: f32, y_lower_inclusive: f32, y_upper_inclusive: f32| -> String {
        let ret = extract_in_aabb0(x_lower_inclusive, x_upper_inclusive, y_lower_inclusive, y_upper_inclusive);
        if (ret.is_empty()) {
            eprintln!("{x_lower_inclusive}..{x_upper_inclusive} * {y_lower_inclusive}..{y_upper_inclusive} does not match");
        }

        ret
    };

    let _please_make_sure_to_read_main_article = extract_in_aabb(74.2f32, 74.3f32, 24.6f32, 24.7f32);
    let subject_personal = extract_in_aabb(20.0f32, 27.0f32, 60.0f32, 70.0f32);
    println!("A. {subject_personal}");
    let subject_houjin = extract_in_aabb(39.0f32, 42.0f32, 60.0f32, 70.0f32);
    println!("B. {subject_houjin}");
    // embedded usage
    {
        let label_embed_video_usage = extract_in_aabb(110.0f32, 116.0f32, 40.0f32, 49.0f32);
        let embed_video = extract_in_aabb(112f32, 113f32, 68f32, 75f32);
        println!("{label_embed_video_usage}: {embed_video}");

        let label_embed_publish_usage = extract_in_aabb(130.0f32, 136.0f32, 40.0f32, 49.0f32);
        let embed_publish = extract_in_aabb(131f32, 132f32, 68f32, 75f32);
        println!("{label_embed_publish_usage}: {embed_publish}");

        let label_embed_goods_usage = extract_in_aabb(149.0f32, 156.0f32, 40.0f32, 49.0f32);
        let embed_goods = extract_in_aabb(150f32, 151f32, 68f32, 75f32);
        println!("{label_embed_goods_usage}: {embed_goods}");

        let label_embed_software_usage = extract_in_aabb(165.0f32, 173.0f32, 40.0f32, 49.0f32);
        let embed_software = extract_in_aabb(168f32, 172f32, 68f32, 75f32);
        println!("{label_embed_software_usage}: {embed_software}");
    }

    // TODO: words.iterが空にならないと全部消費したことにならない
    for rest in words.into_iter().flat_map(|e| e) {
        println!("{rest:?}");
    }

    unimplemented!();
}
