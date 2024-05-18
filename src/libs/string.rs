use regex::Regex;

/// Разбиение строки по сепаратору
/// # Аргументы
/// * `sep` - сепаратор для разбиения строки
/// * `str` - строка, которую нужно разбить
/// # Результат
/// вектор строк
pub fn split(sep: &str, str: String) -> Vec<String> {
  str.split(sep).map(|s| s.to_string()).collect()
}
///
pub fn reg_match(ptrn:&str,txt:String){
  let rx=Regex::new(ptrn).unwrap();
  if let Some(caps)=rx.captures(&txt){
    for cpt in caps.iter() {
      if let Some(mth)=cpt{
        println!("{} s:{} e:{}",mth.as_str(),mth.start(),mth.end());
      }
    }
  }
}