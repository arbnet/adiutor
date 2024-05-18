//#[allow(unused_imports)]
mod libs;
use libs::disc::Path;
use libs::disc::verify;

fn main() {
    let res=verify(Path::new("d:/zzzm"));
    match res {
        Ok(des)=>println!("{}",des),
        Err(err)=>println!("Error: {}", err)
    }
    
    /*let scan=scan_dir("D:/");
    println!("Директории:");
    for txt in &scan.0 {
        println!("{}",txt);
    }
    println!("Файлы:");
    for txt in &scan.1 {
        println!("{}",txt);
    }
    println!("");
    println!("{:?}",scan);*/
}
