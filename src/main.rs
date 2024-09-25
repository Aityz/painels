use std::{
    fs::File,
    io::{copy, BufWriter},
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    println!("Starting download");

    let base_url = "https://storage.googleapis.com/panels-api/data/20240916/media-1a-i-p~s";

    let data: serde_json::Value = reqwest::blocking::get(base_url).unwrap().json().unwrap();

    println!("Fetched version {}", data["version"]);

    let data = data["data"]
        .as_object()
        .unwrap()
        .to_owned()
        .into_iter()
        .collect::<Vec<_>>();

    let _ = std::fs::remove_dir_all("pictures");
    let _ = std::fs::create_dir("pictures");

    data.into_par_iter().for_each(|x| {
        let id = x.0.as_str();

        let _ = std::fs::create_dir(format!("pictures/{}", id));

        let data = x.1;

        let mut i = 0;

        let data = data
            .as_object()
            .unwrap()
            .to_owned()
            .into_iter()
            .collect::<Vec<_>>();

        data.into_iter().for_each(|y| {
            let url = y.1.as_str().unwrap();

            let bytes = reqwest::blocking::get(url).unwrap().bytes().unwrap();

            let mut file =
                BufWriter::new(File::create(format!("pictures/{}/{}.jpg", id, i)).unwrap());

            copy(&mut bytes.as_ref(), &mut file).unwrap();

            println!("Downloaded image: pictures/{}/{}.jpg", id, i);

            i += 1;
        });
    });
}
