# 🔬 ShrinkRay

🌍 [English](./README.md) | [简体中文](./README.zh-CN.md)


**ShrinkRay** is a local image compression tool built with **Tauri 2.0**, offering fast, secure, and efficient compression. It supports **PNG (lossless and lossy)** as well as **JPEG** formats. It can process up to **5 images concurrently** (each up to **20MB** in size) and allows packaging the compressed results into a **ZIP file**. **All processing is done locally, ensuring privacy and security.**

> ⚡ Runs locally, no internet required, safe and reliable!

---

## 🧩 Features

- ✅ Supports PNG (lossy / lossless) and JPEG format compression  
- 🚀 Uses `oxipng` and `pngquant` for efficient compression  
- 🧵 Processes up to 5 images simultaneously  
- 📁 Maximum single image size of 20MB  
- 📦 Supports ZIP packaging of compressed results  
- 🔐 All operations performed locally, ensuring image privacy  
- ⚡ Lightning-fast compression, simple and easy to use

---

## 📦 Installation

Visit the [releases page](https://github.com/charles-lck/shrinkray/releases) to download the installation package for your system. Extract and run it directly.

> 🛠️ This tool is built with [Tauri 2.0](https://v2.tauri.app/). Development requires setting up a Tauri runtime environment.

---

## 🚀 How to Use

Open the app, drag images into the interface, select compression settings, and start compressing!

---

## 🔧 Compression Engines Used

- **PNG (lossless)**: [`oxipng`](https://github.com/shssoichiro/oxipng)  
- **PNG (lossy)**: [`pngquant`](https://github.com/kornelski/pngquant)  
- **JPEG**: Based on an efficient JPEG encoder

---

## 🔐 Security Notes

This tool **runs entirely locally** and does not upload images to any server, making it ideal for handling privacy-sensitive content like design drafts, photos, etc.

---

## 📌 Current Limitations

- ⛔ Maximum single image size: 20MB  
- ⛔ Supports up to 5 images at a time  
- ⛔ Currently supports only PNG and JPEG formats

---

## 👨‍💻 Author

Created with ❤️ by [Charles Luck](https://github.com/charles-lck)  
Feel free to contribute with Issues / PRs / Stars to support the project’s development!


## 📄 License

ShrinkRay is open-sourced under the [MIT License](./LICENSE) — free for personal and commercial use.
