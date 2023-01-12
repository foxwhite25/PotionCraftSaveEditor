use std::fs;
use std::str::from_utf8;

use crate::encoder::{decode, encode};
use crate::meta_data::MetaData;
use crate::potion_data::{PotionData, PotionUsedComponent};
use crate::progress_data::ProgressData;

mod encoder;
mod progress_data;
mod meta_data;
mod potion_data;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // decode <input> or encode <output> or edit <input> <output>
    match args.len() {
        3 => {
            match args[1].as_str() {
                "decode" => {
                    let file = std::fs::read_to_string(&args[2]).expect("Unable to read file");
                    let (meta_data_str, progress_data_str) = file.split_once("\n").unwrap();

                    fs::write("meta.json", from_utf8(&*decode(meta_data_str.as_bytes().to_vec())).unwrap()).expect("Unable to write file meta.json");
                    fs::write("progress.json", from_utf8(&*decode(progress_data_str.as_bytes().to_vec())).unwrap()).expect("Unable to write file progress.json");
                }
                "encode" => {
                    let meta = fs::read("meta.json").expect("Unable to read file meta.json");
                    let progress = fs::read("progress.json").expect("Unable to read file progress.json");

                    let meta_data = encode(meta);
                    let progress_data = encode(progress);

                    fs::write(&args[2], format!("{}\n{}", meta_data, progress_data)).expect("Unable to write file");
                }
                _ => {
                    panic!("Invalid command");
                }
            }
        }
        4 => {
            if &args[1] != "edit" {
                panic!("Invalid arguments");
            }
            let input = &args[2];
            let output = &args[3];
            let file = std::fs::read_to_string(input).unwrap();
            let (meta_data_str, progress_data_str) = file.split_once("\n").unwrap();

            let meta_data = decode(meta_data_str.as_bytes().to_vec());
            let progress_data = decode(progress_data_str.as_bytes().to_vec());

            let meta_data: MetaData = serde_json::from_str(from_utf8(&meta_data).unwrap()).unwrap();
            let mut progress_data: ProgressData = serde_json::from_str(from_utf8(&progress_data).unwrap()).unwrap();

            println!("Save name: {}", meta_data.custom_name.custom_name);
            run_auto_edit(&mut progress_data);

            let modified_progress_data = encode(serde_json::to_string(&progress_data).unwrap().as_bytes().to_vec());

            let save = format!("{}\n{}", meta_data_str, modified_progress_data);
            fs::write(output, save).unwrap();
        }
        _ => {
            panic!("Invalid arguments");
        }
    }
}

fn only_first<T>(v: Vec<T>) -> Vec<T> {
    v.into_iter().enumerate().filter(|(i, _)| *i == 0).map(|(_, x)| x).collect()
}

fn run_auto_edit(data: &mut ProgressData) {
    data.saved_recipes
        .iter_mut()
        .filter(|recipe| {
            recipe.data != ""
        })
        .for_each(|recipe| {
            let potion_data = serde_json::from_str::<PotionData>(&*recipe.data);
            match potion_data {
                Ok(mut potion_data) => {
                    potion_data.potion.used_components_names = only_first(potion_data.potion.used_components_names);
                    potion_data.potion.used_components_amounts = only_first(potion_data.potion.used_components_amounts);
                    potion_data.potion.used_components_types = only_first(potion_data.potion.used_components_types);
                    potion_data.potion_from_panel.potion_used_components = only_first(potion_data.potion_from_panel.potion_used_components);
                    let potion_data = serde_json::to_string(&potion_data).unwrap();
                    recipe.data = potion_data;
                }
                Err(_) => (),
            }
        });
}