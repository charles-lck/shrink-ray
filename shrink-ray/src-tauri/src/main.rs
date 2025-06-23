#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dirs::cache_dir;
use image::ImageOutputFormat;
use imagequant::Attributes;
use oxipng::{optimize, InFile, Options, OutFile};
use png::{ColorType, Encoder};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Semaphore; // 新增 image 库相关

const MAX_FILE_SIZE: u64 = 20 * 1024 * 1024; // 20MB
const MAX_CONCURRENT: usize = 5;

#[derive(serde::Serialize, Clone)]
struct ProgressPayload {
    task_id: String,
    progress: u8,
}

#[tauri::command]
async fn compress_image(
    task_id: String,
    path: String,
    lossless: bool,
    replace: bool,
    app: AppHandle,
    semaphore: tauri::State<'_, Arc<Semaphore>>,
) -> Result<String, String> {
    let input_path = PathBuf::from(&path);
    let app_name = app.package_info().name.clone();
    // 打印app_id
    // println!("App app_name: {}", app_name);
    let cache_dir = cache_dir().ok_or("无法获取系统缓存目录")?;

    let temp_dir = cache_dir.join(format!("com.{}.app/temp", app_name));
    let compressed_dir = cache_dir.join(format!("com.{}.app/compressed", app_name));

    std::fs::create_dir_all(&temp_dir).map_err(|e| format!("创建 temp 目录失败: {}", e))?;
    std::fs::create_dir_all(&compressed_dir)
        .map_err(|e| format!("创建 compressed 目录失败: {}", e))?;

    let output_ext = if input_path.extension().map_or(false, |ext| ext == "png") {
        if lossless {
            "lossless@@.png"
        } else {
            "lossy@@.png"
        }
    } else {
        "lossy@@.jpg" // JPEG 输出始终为 lossy
    };

    let output_path = if replace {
        input_path.clone()
    } else {
        compressed_dir.join(format!(
            "{}@@_{}.{}",
            input_path.file_stem().unwrap_or_default().to_string_lossy(),
            task_id,
            output_ext
        ))
    };

    let temp_path = temp_dir.join(format!(
        "{}_{}.temp.{}",
        input_path.file_stem().unwrap_or_default().to_string_lossy(),
        task_id,
        if input_path.extension().map_or(false, |ext| ext == "png") {
            "png"
        } else {
            "jpg"
        }
    ));

    let metadata =
        std::fs::metadata(&input_path).map_err(|e| format!("无法读取文件元数据: {}", e))?;
    if metadata.len() > MAX_FILE_SIZE {
        return Err(format!("文件大小超过 20MB 限制: {} 字节", metadata.len()));
    }

    let _permit = semaphore
        .acquire()
        .await
        .map_err(|e| format!("获取信号量失败: {}", e))?;

    let result = tauri::async_runtime::spawn_blocking(move || {
        // 根据文件扩展名判断类型
        let is_png = input_path
            .extension()
            .map_or(false, |ext| ext.to_ascii_lowercase() == "png");
        let is_jpeg = input_path.extension().map_or(false, |ext| {
            let ext = ext.to_ascii_lowercase();
            ext == "jpg" || ext == "jpeg"
        });
        if is_png {
            if lossless {
                app.emit(
                    "progress-update",
                    ProgressPayload {
                        task_id: task_id.clone(),
                        progress: 50,
                    },
                )
                .map_err(|e| format!("发送进度失败: {}", e))?;
                let options = Options::default();
                optimize(
                    &InFile::Path(input_path.clone()),
                    &OutFile::Path {
                        path: Some(output_path.clone()),
                        preserve_attrs: true,
                    },
                    &options,
                )
                .map_err(|e| format!("无损压缩失败: {}", e))?;
                app.emit(
                    "progress-update",
                    ProgressPayload {
                        task_id,
                        progress: 100,
                    },
                )
                .map_err(|e| format!("发送进度失败: {}", e))?;
                Ok(output_path.to_string_lossy().to_string())
            } else {
                let input_file =
                    std::fs::File::open(&input_path).map_err(|e| format!("无法打开文件: {}", e))?;
                app.emit(
                    "progress-update",
                    ProgressPayload {
                        task_id: task_id.clone(),
                        progress: 10,
                    },
                )
                .map_err(|e| format!("发送进度失败: {}", e))?;
                let decoder = png::Decoder::new(input_file);
                let mut reader = decoder
                    .read_info()
                    .map_err(|e| format!("读取 PNG 失败: {}", e))?;
                let mut buf = vec![0; reader.output_buffer_size()];
                let info = reader
                    .next_frame(&mut buf)
                    .map_err(|e| format!("解码 PNG 失败: {}", e))?;
                let bytes = &buf[..info.buffer_size()];

                app.emit(
                    "progress-update",
                    ProgressPayload {
                        task_id: task_id.clone(),
                        progress: 20,
                    },
                )
                .map_err(|e| format!("发送进度失败: {}", e))?;

                // 将任意颜色类型转换为 RGBA
                let rgba_pixels: Vec<u8> = match info.color_type {
                    ColorType::Rgba => bytes.to_vec(),
                    _ => {
                        let mut rgba_buf =
                            Vec::with_capacity(info.width as usize * info.height as usize * 4);
                        for pixel in bytes.chunks(info.color_type.samples()) {
                            match info.color_type {
                                ColorType::Grayscale => {
                                    let g = pixel[0];
                                    rgba_buf.extend_from_slice(&[g, g, g, 255]);
                                }
                                ColorType::GrayscaleAlpha => {
                                    let g = pixel[0];
                                    let a = pixel[1];
                                    rgba_buf.extend_from_slice(&[g, g, g, a]);
                                }
                                ColorType::Rgb => {
                                    let r = pixel[0];
                                    let g = pixel[1];
                                    let b = pixel[2];
                                    rgba_buf.extend_from_slice(&[r, g, b, 255]);
                                }
                                ColorType::Indexed => {
                                    // 需要调色板支持，这里简化处理
                                    return Err("暂不支持 Indexed 颜色类型".to_string());
                                }
                                _ => unreachable!(),
                            }
                        }
                        rgba_buf
                    }
                };

                let mut liq = Attributes::new();
                liq.set_max_colors(256)
                    .map_err(|e| format!("设置最大颜色失败: {}", e))?;
                liq.set_quality(0, 90)
                    .map_err(|e| format!("设置质量失败: {}", e))?;

                let rgba_pixel_data: Vec<imagequant::RGBA> = rgba_pixels
                    .chunks_exact(4)
                    .map(|chunk| imagequant::RGBA {
                        r: chunk[0],
                        g: chunk[1],
                        b: chunk[2],
                        a: chunk[3],
                    })
                    .collect();

                let mut img = liq
                    .new_image(
                        rgba_pixel_data,
                        info.width as usize,
                        info.height as usize,
                        0.0,
                    )
                    .map_err(|e| format!("创建图像失败: {}", e))?;
                let mut res = liq
                    .quantize(&mut img)
                    .map_err(|e| format!("量化失败: {}", e))?;
                res.set_dithering_level(1.0)
                    .map_err(|e| format!("设置抖动失败: {}", e))?;

                let (palette, pixels) = res
                    .remapped(&mut img)
                    .map_err(|e| format!("重映射失败: {}", e))?;
                let rgba_output: Vec<u8> = pixels
                    .iter()
                    .flat_map(|&index| {
                        let color = palette[index as usize];
                        [color.r, color.g, color.b, color.a]
                    })
                    .collect();

                let mut temp_file = std::fs::File::create(&temp_path)
                    .map_err(|e| format!("创建临时文件失败: {}", e))?;
                let mut encoder = Encoder::new(&mut temp_file, info.width, info.height);
                encoder.set_color(ColorType::Rgba);
                encoder.set_depth(png::BitDepth::Eight);

                let mut writer = encoder
                    .write_header()
                    .map_err(|e| format!("写入 PNG 头部失败: {}", e))?;
                writer
                    .write_image_data(&rgba_output)
                    .map_err(|e| format!("写入图像数据失败: {}", e))?;
                writer
                    .finish()
                    .map_err(|e| format!("完成 PNG 写入失败: {}", e))?;

                app.emit(
                    "progress-update",
                    ProgressPayload {
                        task_id: task_id.clone(),
                        progress: 90,
                    },
                )
                .map_err(|e| format!("发送进度失败: {}", e))?;

                let temp_metadata = std::fs::metadata(&temp_path)
                    .map_err(|e| format!("检查临时文件失败: {}", e))?;
                if temp_metadata.len() == 0 {
                    return Err("临时文件为空".to_string());
                }

                let options = Options::default();
                optimize(
                    &InFile::Path(temp_path.clone()),
                    &OutFile::Path {
                        path: Some(output_path.clone()),
                        preserve_attrs: true,
                    },
                    &options,
                )
                .map_err(|e| format!("二次优化失败: {}", e))?;

                app.emit(
                    "progress-update",
                    ProgressPayload {
                        task_id,
                        progress: 100,
                    },
                )
                .map_err(|e| format!("发送进度失败: {}", e))?;

                std::fs::remove_file(&temp_path).map_err(|e| format!("删除临时文件失败: {}", e))?;

                Ok(output_path.to_string_lossy().to_string())
            }
        } else if is_jpeg {
            // JPEG 压缩逻辑
            app.emit(
                "progress-update",
                ProgressPayload {
                    task_id: task_id.clone(),
                    progress: 10,
                },
            )
            .map_err(|e| format!("发送进度失败: {}", e))?;

            // 使用 image 库读取 JPEG 文件
            let img = image::open(&input_path).map_err(|e| format!("无法打开 JPEG 文件: {}", e))?;

            app.emit(
                "progress-update",
                ProgressPayload {
                    task_id: task_id.clone(),
                    progress: 50,
                },
            )
            .map_err(|e| format!("发送进度失败: {}", e))?;

            // 设置 JPEG 压缩质量（0-100，100 为最高质量）
            let quality = 75; // 可调整，75 是一个平衡点
            let mut temp_file = std::fs::File::create(&temp_path)
                .map_err(|e| format!("创建临时文件失败: {}", e))?;
            img.write_to(&mut temp_file, ImageOutputFormat::Jpeg(quality))
                .map_err(|e| format!("写入 JPEG 文件失败: {}", e))?;

            app.emit(
                "progress-update",
                ProgressPayload {
                    task_id: task_id.clone(),
                    progress: 90,
                },
            )
            .map_err(|e| format!("发送进度失败: {}", e))?;

            let temp_metadata =
                std::fs::metadata(&temp_path).map_err(|e| format!("检查临时文件失败: {}", e))?;
            if temp_metadata.len() == 0 {
                return Err("临时文件为空".to_string());
            }

            // 将临时文件移动到输出路径
            std::fs::rename(&temp_path, &output_path)
                .map_err(|e| format!("移动文件失败: {}", e))?;

            app.emit(
                "progress-update",
                ProgressPayload {
                    task_id,
                    progress: 100,
                },
            )
            .map_err(|e| format!("发送进度失败: {}", e))?;

            Ok(output_path.to_string_lossy().to_string())
        } else {
            Err("不支持的文件格式，仅支持 PNG 和 JPEG".to_string())
        }
    })
    .await;

    match result {
        Ok(res) => res,
        Err(e) => Err(format!("线程执行失败: {}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Arc::new(Semaphore::new(MAX_CONCURRENT)))
        .invoke_handler(tauri::generate_handler![compress_image])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
