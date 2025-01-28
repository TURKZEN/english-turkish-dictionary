use serde::Deserialize;
use serde_json::{Result};
use std::fs::File;
use std::io::BufReader;

// JSON dosyasındaki her bir kelimenin yapısı
#[derive(Debug, Deserialize)]
struct Entry {
    word: String,
    category: Option<String>,  // category artık opsiyonel bir alan
    #[serde(rename = "type")]
    entry_type: String,
    tr: String,
}

// JSON dosyasını tüm olarak okuyan ve deserialize eden fonksiyon
fn read_json_from_file(file_path: &str) -> Result<Vec<Entry>> {
    let file = File::open(file_path).expect("Dosya açılamadı");
    let reader = BufReader::new(file);
    let entries: Vec<Entry> = serde_json::from_reader(reader)?; // Dosyanın tamamını oku ve deserialize et
    Ok(entries)
}

// Verilen kelimeye göre arama yapan fonksiyon
fn search_word<'a>(entries: &'a [Entry], query: &str) -> Vec<&'a Entry> {
    entries
        .iter()
        .filter(|entry| entry.word.eq_ignore_ascii_case(query))
        .collect()
}

fn main() {
    // Dosya yolunu varsayılan olarak alıyoruz
    let file_path = "/home/turkzen/scripts/dictionary.json";

    // Komut satırından kelime alıyoruz
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Kullanım: {} <kelime>", args[0]);
        return;
    }
    let query = &args[1];

    let entries = match read_json_from_file(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("JSON dosyası parse edilemedi: {:?}", e);
            return;
        }
    };

    let results = search_word(&entries, query);

    if results.is_empty() {
        println!("{} kelimesi bulunamadı.", query);
    } else {
        for entry in results {
            println!(
                "{} ({}) [{}]: {}",
                entry.word,
                entry.category.as_deref().unwrap_or("Belirtilmemiş"),
                entry.entry_type,
                entry.tr
            );
        }
    }
}
