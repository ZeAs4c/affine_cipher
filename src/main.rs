extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};


#[derive(Default, NwgUi)]
pub struct СoderApp {

    // Настроки параметров окна приложения
    #[nwg_control(size: (1000, 700), position: (400, 150), title: "Affine cipher")]
    #[nwg_events( OnWindowClose: [СoderApp::exit] )]
    window: nwg::Window,

    // Настройки окна при выборе файла
    #[nwg_resource(title: "Open File", action: nwg::FileDialogAction::Open, filters: "Txt(*.txt)")]
    dialog: nwg::FileDialog,

    #[nwg_resource(title: "Open File", action: nwg::FileDialogAction::Open, filters: "Crypt(*.crypt)")]
    dialog_ecnr: nwg::FileDialog,

    #[nwg_resource(title: "Open File", action: nwg::FileDialogAction::Open, filters: "Txt(*.txt)|Crypt(*.crypt)")]
    dialog_save: nwg::FileDialog,

    // Настройки кнопок
    #[nwg_control(size: (60, 30), position: (25, 330), text: "Open")]
    #[nwg_events(OnButtonClick: [СoderApp::open_file])]
    open_btn: nwg::Button,

    #[nwg_control(size: (60, 30), position: (425, 330), text: "Open")]
    #[nwg_events(OnButtonClick: [СoderApp::open_file_encr])]
    open_encr_btn: nwg::Button,

    #[nwg_control(size: (60, 30), position: (300, 500), text: "Open")]
    #[nwg_events(OnButtonClick: [СoderApp::open_save_file])]
    open_save_btn: nwg::Button,

    #[nwg_control(size: (150, 30), position: (90, 370), text: "encrypt")]
    #[nwg_events(OnButtonClick: [СoderApp::encrypt])]
    encrypt_btn: nwg::Button,

    #[nwg_control(size: (150, 30), position: (490, 370), text: "decrypt")]
    #[nwg_events(OnButtonClick: [СoderApp::decrypt])]
    decrypt_btn: nwg::Button,

    // Настройки элементов куда вводится путь файла
    #[nwg_control(size: (250, 25), position: (95, 333), readonly: true)]
    file_name: nwg::TextInput,

    #[nwg_control(size: (250, 25), position: (495, 333), readonly: true)]
    file_encr_name: nwg::TextInput,

    #[nwg_control(size: (250, 25), position: (370, 503), readonly: true)]
    file_save_name: nwg::TextInput,

    // Настройки надписей у элементов
    #[nwg_control(size: (85, 25), position: (25, 25), text: "Select key a:")]
    key_a_lbl: nwg::Label,

    #[nwg_control(size: (88, 25), position: (425, 25), text: "Select key a:")]
    key_adc_lbl: nwg::Label,

    #[nwg_control(size: (85, 25), position: (175, 25), text: "Select key b:")]
    key_b_lbl: nwg::Label,

    #[nwg_control(size: (88, 25), position: (575, 25), text: "Select key b:")]
    key_bdc_lbl: nwg::Label,

    #[nwg_control(size: (250, 25), position: (25, 60), text: "Enter the word you want to encrypt:")]
    enter_encr_word_lbl: nwg::Label,

    #[nwg_control(size: (250, 25), position: (425, 60), text: "Encrypted word:")]
    encr_word_lbl: nwg::Label,

    #[nwg_control(size: (250, 25), position: (300, 475), text: "Select the file to save:")]
    save_word_lbl: nwg::Label,

    #[nwg_control(size: (250, 25), position: (25, 300), text: "Select the file to encrypt:")]
    open_file_lbl: nwg::Label,

    #[nwg_control(size: (250, 25), position: (425, 300), text: "Select the encrypted file:")]
    open_file_encr_lbl: nwg::Label,

    // Настройки элементов, которые нужны для ключей шифрования
    #[nwg_control(size: (50, 25), position: (115, 25), collection: СoderApp::generate_vec_a(33))]
    key_a_box: nwg::ComboBox<i32>,

    #[nwg_control(size: (50, 25), position: (515, 25), collection: СoderApp::generate_vec_a(33))]
    key_adc_box: nwg::ComboBox<i32>,

    #[nwg_control(size: (50, 25), position: (265, 25), collection: СoderApp::generate_vec_b(33))]
    key_b_box: nwg::ComboBox<i32>,

    #[nwg_control(size: (50, 25), position: (665, 25), collection: СoderApp::generate_vec_b(33))]
    key_bdc_box: nwg::ComboBox<i32>,

    // Настройки элементов в которые вводится текст, который надо зашифровать и который надо расшифровать
    #[nwg_control(size: (250, 200), position: (25, 90))]
    word_encr_txtin: nwg::TextBox,

    #[nwg_control(size: (250, 200), position: (425, 90), readonly: true)]
    word_dencr_txtin: nwg::TextBox,
}

impl СoderApp {

    // Метод открытия файла, в котором находится текст, который надо зашифровать
    fn open_file(&self) {
        if let Ok(d) = env::current_dir() {
            if let Some(d) = d.to_str() {
                self.dialog
                    .set_default_folder(d)
                    .expect("Failed to set default folder.");
            }
        }

        if self.dialog.run(Some(&self.window)) {
            self.file_name.set_text("");
            if let Ok(directory) = self.dialog.get_selected_item() {
                let dir = directory.into_string().unwrap();
                self.file_name.set_text(&dir);
                self.read_file();
            }
        }
    }

    // Метод открытия файла, в котором находится текст, который надо расшифровать
    fn open_file_encr(&self) {
        if let Ok(d) = env::current_dir() {
            if let Some(d) = d.to_str() {
                self.dialog_ecnr
                    .set_default_folder(d)
                    .expect("Failed to set default folder.");
            }
        }

        if self.dialog_ecnr.run(Some(&self.window)) {
            self.file_encr_name.set_text("");
            if let Ok(directory) = self.dialog_ecnr.get_selected_item() {
                let dir = directory.into_string().unwrap();
                self.file_encr_name.set_text(&dir);
                self.read_file_encr();
            }
        }
    }

    // Метод открытия файла, в который сохраняется результат
    fn open_save_file(&self) {
        if let Ok(d) = env::current_dir() {
            if let Some(d) = d.to_str() {
                self.dialog_save
                    .set_default_folder(d)
                    .expect("Failed to set default folder.");
            }
        }

        if self.dialog_save.run(Some(&self.window)) {
            self.file_save_name.set_text("");
            if let Ok(directory) = self.dialog_save.get_selected_item() {
                let dir = directory.into_string().unwrap();
                self.file_save_name.set_text(&dir);
                self.read_file();
            }
        }
    }

    // Метод генерации вектора чисел, которые взаимно простые к числу 33
    fn generate_vec_a(len: i32) -> Vec<i32> {
        let mut vec = Vec::with_capacity(len as usize);
        for i in 0..len {
            if (i%3 == 0) || (i%11==0){
                continue;
            } else {
                vec.push(i);
            }
        }
        return vec;
    }

    // Метод генерации вектора чисел, которые меньше 33
    fn generate_vec_b(len: i32) -> Vec<i32> {
        let mut vec = Vec::with_capacity(len as usize);
        for i in 0..len {
            vec.push(i);
        }
        return vec;
    }

    // Метод для прочтения файла, в котором находится текст, который надо зашифровать
    fn read_file(&self) -> Result<(), Error> {
        let path = self.file_name.text();
        let input = File::open(path)?;
        let buffered = BufReader::new(input);

        let mut text_file: String = " ".to_string();
        for line in buffered.lines() {
            text_file = line?;
        }
        self.word_encr_txtin.set_text(&*text_file);
        Ok(())
    }

    // Метод для прочтения файла, в котором находится текст, который надо расшифровать
    fn read_file_encr(&self) -> Result<(), Error> {
        let path = self.file_encr_name.text();
        let input = File::open(path)?;
        let buffered = BufReader::new(input);

        let mut text_file: String = " ".to_string();
        for line in buffered.lines() {
            text_file = line?;
        }
        self.word_dencr_txtin.set_text(&*text_file);
        Ok(())
    }

    // Метод для закрытия приложения
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    // Метод алгоритма шифрования "Аффинный шифр"
    fn encrypt(&self) -> Result<(), Error> {

        // Считываем ключи шифрования
        let a: usize = self.key_a_box.selection_string().unwrap().parse().unwrap();
        let b: usize = self.key_b_box.selection_string().unwrap().parse().unwrap();

        // Вектор текста, который нужно зашифровать
        let word_vec: Vec<char> = self.word_encr_txtin.text().chars().collect();

        // Массивы алфавита
        let alphabet: [char; 33] = ['а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я'];
        let big_alphabet: [char; 33] = ['А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я'];

        // Массив, который нужен для цифр, которые будут соответсвовать каждой букве алфавита
        let mut shifr: [usize; 33] = [0; 33];

        // Вектор, который нужен для зашифрованного текста
        let mut word_encr_vec: Vec<char> = self.word_encr_txtin.text().chars().collect();

        // Счетчики
        let mut i:usize = 0;
        let mut j:usize = 0;

        // Заполнение массива цифрами по формуле шифрования "Аффинного шифра"
        while i< alphabet.len() {
            shifr[i] = (a*j+b)%33;
            j+=1;
            i+=1;
        }
        i=0;

        // Счетчик
        let mut k:usize = 0;

        // Сверяем буквы текста, который надо зашифровать с алфавитом, и присваиваем букву по цифрам(которые искали по формуле) финальному массиву
        while i<word_vec.len(){
            while  k<alphabet.len() {
                if word_vec[i] == alphabet[k] {
                    word_encr_vec[i] = alphabet[shifr[k]];
                    k+=1;
                } else if word_vec[i] == big_alphabet[k] {
                    word_encr_vec[i] = big_alphabet[shifr[k]];
                    k+=1;
                } else {
                    k+=1;
                }
            }
            k=0;
            i+=1;
        }

        // Смотрим путь файла, куда будем сохранять наш зашифрованный текст
        let path = self.file_save_name.text();

        // Открываем файл для записи
        let mut output = File::create(path)?;

        // Вектор зашифрованного текста переводим в строку
        let s: String = word_encr_vec.into_iter().collect();

        // Записываем
        write!(output,"{}",s)?;

        // Выводим сообщение, что текст зашифрован
        nwg::simple_message("Сообщение", &format!("Текст зашифрован"));

        // Возращаем результат
        Ok(())
    }

    // Метод алгоритма расшифрования "Аффинный шифр"
    fn decrypt(&self) -> Result<(), Error> {

        // Считываем ключи шифрования
        let a: usize = self.key_adc_box.selection_string().unwrap().parse().unwrap();
        let mut a1: usize = a;
        let b: usize = self.key_bdc_box.selection_string().unwrap().parse().unwrap();

        // Вектор текста, который нужно расшифровать
        let word_vec: Vec<char> = self.word_dencr_txtin.text().chars().collect();

        // Вектор цифр, которые взаимно просты числу 33
        let word_vec_a: [usize; 20] = [1,2,4,5,7,8,10,13,14,16,17,19,20,23,25,26,28,29,31,32];

        // Находим нужное число которое будет взаимно просто числу 33 и умножение ключа на это число, и найдя остаток от деления этого произведения на 33 будет давать нам 1
        for i in word_vec_a {
            if (a1*i)%33 == 1 {
                a1= i;
                break;
            }
        }

        // Массивы алфавита
        let alphabet: [char; 33] = ['а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я'];
        let big_alphabet: [char; 33] = ['А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я'];

        // Массив, который нужен для цифр, которые будут соответсвовать каждой букве алфавита
        let mut shifr: [usize; 33] = [0; 33];

        // Вектор, который нужен для расшифрованного текста
        let mut word_encr_vec: Vec<char> = self.word_dencr_txtin.text().chars().collect();

        // Счетчики
        let mut i:usize = 0;
        let mut j:usize = 0;

        // Заполнение массива цифрами по формуле расшифрования "Аффинного шифра"
        while i< alphabet.len() {
            shifr[i] = a1*(33+j-b)%33;
            j+=1;
            i+=1;
        }
        i=0;

        // Счетчик
        let mut k:usize = 0;

        // Сверяем буквы текста, который надо расшифровать с алфавитом, и присваиваем букву по цифрам(которые искали по формуле) финальному массиву
        while i<word_vec.len(){
            while  k<alphabet.len() {
                if word_vec[i] == alphabet[k] {
                    word_encr_vec[i] = alphabet[shifr[k]];
                    k+=1;
                } else if word_vec[i] == big_alphabet[k] {
                    word_encr_vec[i] = big_alphabet[shifr[k]];
                    k+=1;
                } else {
                    k+=1;
                }
            }
            k=0;
            i+=1;
        }

        // Смотрим путь файла, куда будем сохранять наш расшифрованный текст
        let path = self.file_save_name.text();

        // Открываем файл для записи
        let mut output = File::create(path)?;

        // Вектор расшифрованного текста переводим в строку
        let s: String = word_encr_vec.into_iter().collect();

        // Запись в файл
        write!(output,"{}",s)?;

        // Вывод сообщения, что текст расшифрован
        nwg::simple_message("Сообщение", &format!("Текст расшифрован"));

        // Возращение результата функции
        Ok(())
    }
}

// Основная функция для запуска и работы приложения
fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = СoderApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
