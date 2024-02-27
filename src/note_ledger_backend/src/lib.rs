use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};
use tesseract::Tesseract;

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

    fn add_note_from_image(&mut self, title: String, image_path: &str) {
        let content = process_image(image_path);
        let note = Note::new(title, content);
        self.add_note(note);
    }
}

fn process_image(image_path: &str) -> String {
    // Resmi oku
    let img = ImageReader::open(image_path)
        .unwrap()
        .decode()
        .expect("Resim açılamadı");

    // Resmi metne dönüştür
    let mut tesseract = Tesseract::new();
    tesseract.set_image(&img);
    tesseract.set_language("eng"); // OCR dilini İngilizce olarak ayarladık
    tesseract.get_text().expect("OCR hatası")
}

fn main() {
    let mut note_manager = NoteManager::new();

    loop {
        println!("1. Not Ekle");
        println!("2. Notları Listele");
        println!("3. Not Sil");
        println!("4. Fotoğraftan Not Al");
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
            }
            Ok(2) => {
                note_manager.list_notes();
            }
            Ok(3) => {
                println!("Silmek istediğiniz notun numarasını girin:");
                let mut index_input = String::new();
                io::stdin().read_line(&mut index_input).expect("Okuma hatası");

                let index: usize = match index_input.trim().parse::<usize>() {
                    Ok(num) => num - 1,
                    Err(_) => {
                        println!("Geçersiz indeks");
                        continue;
                    }
                };

                if let Some(note) = note_manager.remove_note(index) {
                    println!("{} başlıklı not silindi.", note.title);
                } else {
                    println!("Geçersiz indeks");
                }
            }
            Ok(4) => {
                println!("Resmin dosya yolunu girin:");
                let mut image_path = String::new();
                io::stdin().read_line(&mut image_path).expect("Okuma hatası");

                let title = String::from("Not (Fotoğraf)");

                note_manager.add_note_from_image(title, image_path.trim());
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
