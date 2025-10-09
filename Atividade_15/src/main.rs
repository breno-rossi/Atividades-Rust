use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
enum DownloaderError {
    #[error("URL inválida: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Erro de requisição HTTP: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Erro de I/O: {0}")]
    IoError(#[from] io::Error),

    #[error("Falha ao parsear o HTML")]
    HtmlParseError,

    #[error("Nenhuma imagem encontrada na página")]
    NoImagesFound,
}

type Result<T> = std::result::Result<T, DownloaderError>;

#[derive(Debug, Clone)]
struct ImageInfo {
    url: Url,
    filename: String,
}

impl ImageInfo {
    fn new(url: Url) -> Self {
        let filename = url
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("image")
            .to_string();

        Self { url, filename }
    }

    fn is_image(&self) -> bool {
        let valid_extensions = [".jpg", ".jpeg", ".png", ".gif", ".webp", ".bmp", ".svg"];
        let filename_lower = self.filename.to_lowercase();

        valid_extensions.iter().any(|ext| filename_lower.ends_with(ext))
    }
}

struct ImageDownloader {
    client: Client,
    download_dir: String,
}

impl ImageDownloader {
    fn new(download_dir: &str) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Falha ao criar cliente HTTP"),
            download_dir: download_dir.to_string(),
        }
    }

    fn validate_url(&self, url_str: &str) -> Result<Url> {
        let url = Url::parse(url_str)?;

        if url.scheme() != "http" && url.scheme() != "https" {
            return Err(DownloaderError::InvalidUrl(
                url::ParseError::RelativeUrlWithoutBase
            ));
        }

        Ok(url)
    }

    fn download_page(&self, url: &Url) -> Result<String> {
        println!(" Baixando página: {}", url);

        let response = self.client.get(url.as_str()).send()?;

        if !response.status().is_success() {
            return Err(DownloaderError::HttpError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let html = response.text()?;
        println!(" Página baixada com sucesso ({} bytes)", html.len());

        Ok(html)
    }

    fn extract_image_urls(&self, html: &str, base_url: &Url) -> Result<Vec<ImageInfo>> {
        println!("🔍 Extraindo links de imagens...");

        let document = Html::parse_document(html);
        let img_selector = Selector::parse("img")
            .map_err(|_| DownloaderError::HtmlParseError)?;

        let mut images = Vec::new();

        for element in document.select(&img_selector) {
            if let Some(src) = element.value().attr("src") {
                if let Ok(absolute_url) = base_url.join(src) {
                    let image_info = ImageInfo::new(absolute_url);

                    if image_info.is_image() {
                        images.push(image_info);
                    }
                }
            }
        }

        if images.is_empty() {
            return Err(DownloaderError::NoImagesFound);
        }

        println!(" {} imagem(ns) encontrada(s)", images.len());
        Ok(images)
    }

    fn download_image(&self, image: &ImageInfo, index: usize) -> Result<()> {
        println!("  [{}/...] Baixando: {}", index, image.filename);

        let response = self.client.get(image.url.as_str()).send()?;

        if !response.status().is_success() {
            println!("Falha ao baixar (status: {})", response.status());
            return Ok(());
        }

        let bytes = response.bytes()?;

        let filepath = Path::new(&self.download_dir).join(&image.filename);
        let mut file = File::create(&filepath)?;
        file.write_all(&bytes)?;

        println!("Salva: {} ({} bytes)", filepath.display(), bytes.len());

        Ok(())
    }

    fn download_images(&self, images: Vec<ImageInfo>) -> Result<usize> {
        fs::create_dir_all(&self.download_dir)?;

        println!("\n📦 Iniciando download de {} imagem(ns)...\n", images.len());

        let mut success_count = 0;

        for (index, image) in images.iter().enumerate() {
            match self.download_image(image, index + 1) {
                Ok(_) => success_count += 1,
                Err(e) => println!("Erro: {}", e),
            }
        }

        Ok(success_count)
    }

    fn run(&self, url_str: &str) -> Result<()> {
        let url = self.validate_url(url_str)?;
        let html = self.download_page(&url)?;
        let images = self.extract_image_urls(&html, &url)?;
        let downloaded = self.download_images(images)?;

        println!("\n Download concluído!");
        println!("   {} imagem(ns) salva(s) em: {}", downloaded, self.download_dir);

        Ok(())
    }
}

fn main() {
    println!("=== Image Downloader ===\n");

    print!("Digite a URL da página: ");
    io::stdout().flush().expect("Falha ao limpar buffer");

    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("Falha ao ler entrada");

    let url = url.trim();

    if url.is_empty() {
        eprintln!("Erro: URL não pode estar vazia");
        return;
    }

    let downloader = ImageDownloader::new("./downloads");

    match downloader.run(url) {
        Ok(_) => println!("\n Operação finalizada com sucesso!"),
        Err(e) => eprintln!("\n Erro: {}", e),
    }
}