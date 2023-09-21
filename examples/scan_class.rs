use dex::DexReader;
// 扫描dex内的class信息
fn main() {
    let dex = DexReader::from_file("resources/0b75e3710db9c2d897c5d9d9a9728cfe.dex").unwrap();
    // 遍历所有的类信息
    // dex.
    for class in dex.classes() {
        let class_info = class.unwrap();
        let class_name = class_info
            .get_class_name_pretty(&dex)
            .unwrap_or("".to_string());
        // 查找指定类的信息并输出方法信息
        if class_name != "com.tencent.assistant.utils.DeviceUtils" {
            continue;
        }
        println!("class_name = {class_name:?}");
        println!("soruce File = {:?}", class_info.source_file());
        for method in class_info.methods() {
            let signature = method.method_name_pretty().unwrap();
            println!("method_name = {signature:?}");
        }

        println!("")
    }
}
