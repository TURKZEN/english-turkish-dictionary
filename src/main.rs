use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::{BufReader, Write, Read}; // Buraya `Read` eklendi!
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};

// JSON dosyasındaki her bir kelimenin yapısı
#[derive(Debug, Deserialize)]
struct Entry {
    word: String,
    category: Option<String>, // category artık opsiyonel bir alan
    #[serde(rename = "type")]
    entry_type: String,
    tr: String,
}

// GitHub'dan JSON dosyasını indir ve eğer dosya mevcutsa indirme
fn download_json_file_if_not_exists(url: &str, save_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(save_path);

    // Dosya zaten mevcut mu?
    if path.exists() {
        return Ok(()); // Dosya varsa indirmeye gerek yok
    }

    println!("Sözlük indiriliyor:");

    // İstek başlat
    let mut response = reqwest::blocking::get(url)?.error_for_status()?; // URL'yi indir

    // İndirilen dosyanın toplam boyutunu alın
    let total_size = response.content_length().unwrap_or(0);

    // Progress bar oluştur
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .expect("Progress bar template error")
            .progress_chars("#>-"),
    );

    // Dosyayı indirin ve progress bar'ı güncelleyerek yerel diske kaydedin
    let mut file = File::create(save_path)?;
    let mut buffer = vec![0; 8 * 1024]; // 8 KB'lık buffer
    let mut downloaded = 0;

    loop {
        let read_bytes = response.read(&mut buffer)?; // `read` için `std::io::Read` gerekiyor
        if read_bytes == 0 {
            break; // Veri kalmadıysa döngüyü sonlandır
        }

        file.write_all(&buffer[..read_bytes])?;
        downloaded += read_bytes as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Sözlük başarıyla indirildi!");
 
    Ok(())
}

// JSON dosyasını tüm olarak okuyan ve deserialize eden fonksiyon
fn read_json_from_file(file_path: &str) -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // JSON dosyasının kaydedileceği yerel dosya adı
    let file_path = "./dictionary.json";

    // JSON dosyasının bulunduğu GitHub URL'si
    let file_url = "https://github.com/TURKZEN/english-turkish-dictionary/raw/refs/heads/main/dictionary.json?download=";

    // JSON dosyasını indir (eğer gerekliyse)
    download_json_file_if_not_exists(file_url, file_path)?;

    // Komut satırından kelime alıyoruz
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Kullanım: {} <kelime>", args[0]);
        return Ok(());
    }
    let query = &args[1];

    // JSON dosyasını oku
    let entries = read_json_from_file(file_path)?;

    // Kelimeyi ara
    let results = search_word(&entries, query);

    // Sonuçları yazdır
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

    Ok(())
}
