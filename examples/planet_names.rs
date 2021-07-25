// Planet name generation based on https://github.com/hbi99/namegen

use std::{collections::HashMap, env, fs};

use rand::Rng;

fn main() {
    let num_names = env::args().skip(1).next().unwrap().parse().unwrap();

    let vowels_json = fs::read_to_string("examples/vowels.json").expect("Unable to read file");
    let vowels_data: serde_json::Value =
        serde_json::from_str(&vowels_json).expect("Unable to parse");
    let mut vowels: HashMap<String, Vec<&str>> = Default::default();
    vowels_data.as_object().unwrap().iter().for_each(|(k, v)| {
        vowels.insert(
            k.clone(),
            v.as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect(),
        );
    });

    let mtx_json = fs::read_to_string("examples/mtx.json").expect("Unable to read file");
    let mtx_data: serde_json::Value = serde_json::from_str(&mtx_json).expect("Unable to parse");
    let mut mtx: Vec<Vec<usize>> = Default::default();
    mtx_data.as_array().unwrap().iter().for_each(|v| {
        mtx.push(
            v.as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap() as usize)
                .collect(),
        )
    });

    let mut rng = rand::thread_rng();
    let mut names: Vec<String> = Default::default();

    for c in 0..num_names {
        let mut name = String::default();
        let comp = &(mtx[c % mtx.len()]);

        for i in 0..comp.len() / 2 {
            let comp_index = comp[i * 2 + 1].to_string();
            let vowel_index: usize = rng.gen_range(0, vowels[&comp_index].len());
            name.push_str(vowels[&comp[i * 2].to_string()][vowel_index]);
        }

        names.push(name);
    }

    names.iter().for_each(|n| println!("{}", n));
}
