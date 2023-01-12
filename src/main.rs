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
    let file = std::fs::read_to_string("save.pcsave").unwrap();
    let (meta_data_str, progress_data_str) = file.split_once("\n").unwrap();

    let meta_data = decode(meta_data_str.as_bytes().to_vec());
    let progress_data = decode(progress_data_str.as_bytes().to_vec());

    let meta_data: MetaData = serde_json::from_str(from_utf8(&meta_data).unwrap()).unwrap();
    let mut progress_data: ProgressData = serde_json::from_str(from_utf8(&progress_data).unwrap()).unwrap();

    println!("Save name: {}", meta_data.custom_name.custom_name);
    progress_data.saved_recipes
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
                    (*recipe).data = potion_data;
                }
                Err(_) => (),
            }
        });

    let modified_progress_data = serde_json::to_string(&progress_data).unwrap();
    let modified_progress_data = encode(modified_progress_data.as_bytes().to_vec());

    let save = format!("{}\n{}", meta_data_str, modified_progress_data);
    fs::write("save1.pcsave", save).unwrap();
}

fn only_first<T>(v: Vec<T>) -> Vec<T> {
    v.into_iter().enumerate().filter(|(i, _)| *i == 0).map(|(_, x)| x).collect()
}