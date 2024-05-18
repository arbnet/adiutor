use std::fs;
pub use std::path::Path;
use std::io::{BufRead,BufReader};

/// Проверка пути
/// ### Аргументы
/// * `pth` - путь к директории
/// ### Результат
/// `Result` directory, file или ошибка
pub fn verify(pth:&Path)->Result<String,String>{
  match fs::metadata(pth) {
    Ok(mdt) => {
      if mdt.is_dir() {Ok("directory".to_string())}
      else if mdt.is_file() {Ok("file".to_string())}
      else{Err("unknown".to_string())}
    }
    Err(_) =>Err("does not exist".to_string())
  }
}
/// Создание директории
/// ### Аргументы
/// * `pth` - путь к директории
/// ### Результат
/// `true` директория создана, иначе `false`
pub fn new_dir(pth:&Path)->bool{
  match fs::create_dir(pth) {
    Ok(_) =>true, Err(_)=>false
  }
}
/// Удаление директории
/// ### Аргументы
/// * `pth` - путь к директории
/// ### Результат
/// `true` директория создана, иначе `false`
pub fn delete_dir(pth:&Path)->bool{
  match fs::remove_dir_all(pth) {
    Ok(_) =>true, Err(_) => false
  }
}
/// Сканирование директории
/// ### Аргументы
/// * `pth` - путь к директории
/// ### Результат
/// картеж (список директорий, список файлов)
pub fn scan_dir(pth:&Path)->(Vec<String>,Vec<String>){
  let mut vdir=Vec::new();
  let mut vfls=Vec::new();
  let scan = fs::read_dir(pth).unwrap();
  for item in scan {
    if let Ok(entry) = item {
      if entry.path().is_dir() {
        if let Some(name) = entry.file_name().into_string().ok() {
          vdir.push(name);
        }
      }else{
        if let Some(name) = entry.file_name().into_string().ok() {
          vfls.push(name);
        }
      }
    }
  }
  (vdir,vfls)
}
/// Загрузка текста из файла
/// ### Аргументы
/// * `pth` - путь к файлу
/// ### Результат
/// вектор строк с текстом файла
pub fn load_text(pth:&Path)->Vec<String>{
  let file = fs::File::open(pth).unwrap();
  let reader = BufReader::new(file);
  let mut lines: Vec<String> = Vec::new();
  for line in reader.lines() {
    match line {
      Ok(txt) => lines.push(txt),
      Err(err) => eprintln!("Error load_text line: {}",err),
    }
  }
  lines
}




/*
pub fn is_dir(dir:String)->bool{
  let path = Path::new(&dir);
  path.is_dir()
}
*/