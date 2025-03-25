use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use zip::ZipArchive;

#[derive(Debug, Clone, Deserialize)]
struct Wiki {
    //id: String,
    text: String,
    title: String,
}
struct DeScris {
    tot_frecventa: HashMap<String, u32>,
    tot_frecventamica: HashMap<String, u32>,
    max_article: String,
    max_path_article: String,
    size_article: usize,
    max_title: String,
    max_path_title: String,
    size_title: usize,
}
fn procesare(
    content: &Vec<Wiki>,
) -> (
    HashMap<String, u32>,
    HashMap<String, u32>,
    usize,
    String,
    usize,
    String,
) {
    let mut frecventa = HashMap::<String, u32>::new();
    let mut frecventamica = HashMap::<String, u32>::new();
    let mut titlu_max: usize = 0;
    let mut titlu_maxstr: String = String::new();
    let mut article_max: usize = 0;
    let mut article_maxstr: String = String::new();
    for i in content {
        let nr_titlu = i.title.chars().count();
        let nr_articol = i.text.chars().count();
        if nr_titlu > titlu_max {
            titlu_max = nr_titlu;
            titlu_maxstr = i.title.clone();
        }
        if nr_articol > article_max {
            article_max = nr_articol;
            article_maxstr = i.title.clone();
        }
        for j in i
            .text
            .split(|cuv: char| !cuv.is_alphabetic())
            .filter(|cuv| !cuv.is_empty())
        {
            let mut key = j.to_string();
            frecventa
                .entry(key.clone())
                .and_modify(|x| {
                    *x += 1;
                })
                .or_insert(1);
            key = key.to_ascii_lowercase();
            frecventamica
                .entry(key.clone())
                .and_modify(|x| {
                    *x += 1;
                })
                .or_insert(1);
        }
    }
    (
        frecventa,
        frecventamica,
        titlu_max,
        titlu_maxstr,
        article_max,
        article_maxstr,
    )
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut data: DeScris = DeScris {
        tot_frecventa: HashMap::<String, u32>::new(),
        tot_frecventamica: HashMap::<String, u32>::new(),
        max_article: String::new(),
        max_path_article: String::new(),
        size_article: 0,
        max_title: String::new(),
        max_path_title: String::new(),
        size_title: 0,
    };

    let start = Instant::now();
    let folder = File::open("./enwiki20201020.zip")?;
    let mut arhiva = ZipArchive::new(folder)?;
    for i in 0..arhiva.len() {
        let mut json = match arhiva.by_index(i) {
            Ok(file) => file,
            Err(err) => {
                println!("Nu am putut deschide jsonul nr: {}", i);
                println!("{:?}", err);
                continue;
            }
        };
        let content: Vec<Wiki> = match serde_json::from_reader(&mut json) {
            Ok(file) => file,
            Err(err) => {
                println!("Nu am accesa continutul: {}", i);
                println!("{:?}", err);
                continue;
            }
        };
        let (locf, locfmic, titlu, titlu_str, article, article_str) = procesare(&content);
        if titlu > data.size_title {
            data.size_title = titlu;
            let r = json.enclosed_name();
            match r {
                Some(value) => {
                    data.max_path_title.clear();
                    data.max_path_title
                        .push_str(value.to_string_lossy().as_ref())
                }
                None => println!("Fisier necunoscut!"),
            }
            data.max_title = titlu_str;
        }
        if article > data.size_article {
            data.size_article = article;
            let r = json.enclosed_name();
            match r {
                Some(value) => {
                    data.max_path_article.clear();
                    data.max_path_article
                        .push_str(value.to_string_lossy().as_ref())
                }
                None => println!("Fisier necunoscut!"),
            }
            data.max_article = article_str;
        }
        for it in locf {
            data.tot_frecventa
                .entry(it.0.clone())
                .and_modify(|x| {
                    *x += it.1;
                })
                .or_insert(it.1);
        }
        for it in locfmic {
            data.tot_frecventamica
                .entry(it.0.clone())
                .and_modify(|x| {
                    *x += it.1;
                })
                .or_insert(it.1);
        }
    }
    scrie(&data)?;

    let t = start.elapsed();
    println!("Timpu: {:?}", t);
    Ok(())
}

fn scrie(data: &DeScris) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = File::create("output.txt")?;
    let mut output_str = String::new();
    output_str.push_str("Frecventa cuvinte:\n");
    for it in data.tot_frecventa.clone() {
        output_str.push_str(&format!("Cuvant: {} Frecventa: {}\n", it.0, it.1));
    }

    output_str.push_str("Frecventa cuvinte_lower:\n");
    for it in data.tot_frecventamica.clone() {
        output_str.push_str(&format!("Cuvant: {} Frecventa: {}\n", it.0, it.1));
    }

    output_str.push_str("Cel mai lung articol:\n");
    output_str.push_str(&format!(
        "Titlu:{:?}; Path: ./enwiki20201020.zip/{:?}; Lungime:{};\n",
        data.max_article, data.max_path_article, data.size_article
    ));

    output_str.push_str("Cel mai lung titlu:\n");
    output_str.push_str(&format!(
        "Titlu:{:?}; Path: ./enwiki20201020.zip/{:?}; Lungime:{};\n",
        data.max_title, data.max_path_title, data.size_title
    ));

    output.write_all(output_str.as_bytes())?;
    Ok(())
}
