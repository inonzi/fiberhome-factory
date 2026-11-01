use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;

use fiberhome_factory::model::{Factory, Result};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("用法: {} <输入镜像.bin> <输出文件.bin>", args[0]);
        process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_path = PathBuf::from(&args[2]);

    println!("正在读取: {}", input_path.display());
    
    let opened = match Factory::open(&input_path) {
        Ok(opened) => opened,
        Err(e) => {
            eprintln!("解析失败: {}", e);
            process::exit(1);
        }
    };

    if let Some(warning) = opened.warning {
        eprintln!("警告: 原厂数据存在问题 {:?}", warning);
    }

    let data = match opened.factory.encode() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("编码 1MiB 失败: {}", e);
            process::exit(1);
        }
    };

    match save_image(&output_path, &data) {
        Ok(_) => {
            println!("成功导出: {}", output_path.display());
            println!("文件大小: {} 字节", data.len());
        }
        Err(e) => {
            eprintln!("写入文件失败: {}", e);
            process::exit(1);
        }
    }
}

fn save_image(path: &Path, data: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut tmp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    tmp.write_all(data)
        .and_then(|_| tmp.as_file().sync_all())
        .map_err(|e| e.to_string())?;
    tmp.persist(path).map_err(|e| e.to_string())?;
    Ok(())
}
