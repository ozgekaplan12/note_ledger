use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;

use image::{self, DynamicImage, GenericImageView};

struct Note {
    title: String,
    content: String,
}

impl Note {
    fn new(title: String, content: String) -> Self {
        Self { title, content }
    }

    fn display(&self) {
        println!("--- {} ---\n{}", self.title, self.content);
    }
}

struct NoteManager {
    notes: Vec<Note>,
}

impl NoteManager {
    fn new() -> Self {
        Self { notes: Vec::new() }
    }

    fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    fn remove_note(&mut self, index: usize) -> Option<Note> {
        if index < self.notes.len() {
            Some(self.notes.remove(index))
        } else {
            None
        }
    }

    fn list_notes(&self) {
        for (index, note) in self.notes.iter().enumerate() {
            println!("{}. {}", index + 1, note.title);
        }
    }
}

fn main() {
    let mut note_manager = NoteManager::new();

    loop {
        println!("1. Not Ekle");
        println!("2. Fotoğraf Ekle");
        println!("3. Notları Listele");
        println!("4. Not Sil");
        println!("5. Çıkış");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Okuma hatası");

        match choice.trim().parse() {
            Ok(1) => {
                println!("Not başlığını girin:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Okuma hatası");

                println!("Not içeriğini girin:");
                let mut content = String::new();
                io::stdin().read_line(&mut content).expect("Okuma hatası");

                let note = Note::new(title.trim().to_string(), content.trim().to_string());
                note_manager.add_note(note);
                println!("Not başarıyla eklendi.");
            }
            Ok(2) => {
                println!("1. Fotoğraf çek");
                println!("2. Dosyadan yükle");
                let mut img_choice = String::new();
                io::stdin().read_line(&mut img_choice).expect("Okuma hatası");

                match img_choice.trim() {
                    "1" => {
                        unimplemented!("Fotoğraf çekme işlemi henüz implemente edilmedi.");
                    }
                    "2" => {
                        println!("Fotoğraf dosyasının yolunu girin:");
                        let mut file_path = String::new();
                        io::stdin().read_line(&mut file_path).expect("Okuma hatası");

                        let image = open_image(&file_path.trim());
                        if let Some(image) = image {
                            let text = extract_text_from_image(&image);
                            let note = Note::new("Fotoğraf Notu".to_string(), text);
                            note_manager.add_note(note);
                            println!("Fotoğraf notu başarıyla eklendi.");
                        } else {
                            println!("Fotoğraf yüklenirken bir hata oluştu.");
                        }
                    }
                    _ => {
                        println!("Geçersiz seçenek");
                    }
                }
            }
            Ok(3) => {
                note_manager.list_notes();
            }
            Ok(4) => {
                println!("Silinmesi gereken notun numarasını girin:");
                let mut index_input = String::new();
                io::stdin().read_line(&mut index_input).expect("Okuma hatası");
                let index = index_input.trim().parse::<usize>().expect("Geçersiz indeks");
                if let Some(note) = note_manager.remove_note(index - 1) {
                    println!("{} notu başarıyla silindi.", note.title);
                } else {
                    println!("Belirtilen indekse sahip bir not bulunamadı.");
                }
            }
            Ok(5) => {
                println!("Çıkılıyor...");
                break;
            }
            _ => {
                println!("Geçersiz seçenek");
            }
        }
    }
}

fn open_image(file_path: &str) -> Option<DynamicImage> {
    match image::open(file_path) {
        Ok(image) => Some(image),
        Err(err) => {
            eprintln!("Fotoğraf açılırken bir hata oluştu: {}", err);
            None
        }
    }
}

fn extract_text_from_image(image: &DynamicImage) -> String {
    let temp_file_path = "temp_image.jpg";

    if let Err(err) = image.save(temp_file_path) {
        eprintln!("Fotoğraf kaydedilirken bir hata oluştu: {}", err);
        return String::new();
    }

    let output = Command::new("tesseract")
        .args(&[temp_file_path, "stdout"])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    return stdout.trim().to_string();
                }
            } else {
                eprintln!("Tesseract-OCR hata çıkışı: {:?}", output.stderr);
            }
        }
        Err(err) => {
            eprintln!("Tesseract-OCR çalıştırılırken bir hata oluştu: {}", err);
        }
    }

    String::new()
}
