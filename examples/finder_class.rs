use dex::class::INVALID_CLASS_ID;
use dex::DexReader;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

fn dump_class(path: &PathBuf) {
    let dex = DexReader::from_file(path).unwrap();
    for class in dex.classes() {
        let class_info = class.unwrap_or_default();
        if class_info.id() == INVALID_CLASS_ID {
            // println!("Parse class Error");
            continue;
        }
        let class_name = class_info
            .get_class_name_pretty(&dex)
            .unwrap_or("".to_string());
        // 查找指定类的信息并输出方法信息
        if class_name.contains("IPCDeviceInfo")
            || class_name.contains("AstApp")
            || class_name.contains("IPCSafeDeviceInfo")
            || class_name == "f8.a"
            || class_name == "f8.b"
            //先不用找个来做
            // || class_name.contains("DeviceU")
            || class_name.contains("com.tencent.qmethod.pandoraex.monitor")
        {
            println!(
                "-> found {:?} class name = {}",
                path.file_name().unwrap(),
                class_name
            );
        }
    }
}
//扫描给定目录并输出类信息
fn read_class() -> Result<()> {
    let dir = r"D:\研究应用宝保活技术\dexes";
    for entry in fs::read_dir(dir)? {
        // 读取指定的dex文件
        let entry = entry?;
        let data = entry.metadata()?;
        let path = entry.path();
        if data.is_file() {
            if let Some(ex) = path.extension() {
                if ex == "dex" && data.len() > 0 {
                    println!("{} length {}", path.display(), data.len());
                    dump_class(&path);
                }
            }
        }
    }
    Ok(())
}
fn main() {
    println!("run finder class");
    let _ = read_class();
}
