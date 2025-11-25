use anyhow::{anyhow, bail, Context, Result};
use chrono::NaiveDate;
use liquid::model::{KString, Value};
use liquid::{Object, Parser, ParserBuilder, ValueView};
use pulldown_cmark::{html, Options, Parser as MdParser};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// =============================================================================
// Configuration
// =============================================================================

#[derive(Debug, Deserialize)]
struct ConfigFile {
    title: Option<String>,
    author: Option<Author>,
    url: Option<String>,
    permalink: Option<String>,
    google_analytics: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Author {
    name: Option<String>,
    email: Option<String>,
    github: Option<String>,
}

#[derive(Debug, Clone)]
struct SiteConfig {
    title: String,
    author: Author,
    url: String,
    permalink: String,
    google_analytics: Option<String>,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            author: Author {
                name: None,
                email: None,
                github: None,
            },
            url: String::new(),
            permalink: "/:categories/:year/:month/:day/:title".to_string(),
            google_analytics: None,
        }
    }
}

// =============================================================================
// Content Types
// =============================================================================

#[derive(Debug, Clone)]
struct Post {
    title: String,
    date: NaiveDate,
    slug: String,
    categories: Vec<String>,
    tags: Vec<String>,
    content_html: String,
    layout: String,
    url: String,
}

#[derive(Debug, Clone)]
struct Page {
    title: String,
    output_path: PathBuf,
    content_html: String,
    layout: String,
    url: String,
    is_html: bool,
}

#[derive(Debug, Deserialize, Default)]
struct FrontMatter {
    layout: Option<String>,
    title: Option<String>,
    tags: Option<Vec<String>>,
}

// =============================================================================
// Site Builder
// =============================================================================

struct SiteBuilder {
    source_dir: PathBuf,
    output_dir: PathBuf,
    config: SiteConfig,
    layouts: HashMap<String, String>,
    posts: Vec<Post>,
    pages: Vec<Page>,
    liquid: Parser,
}

impl SiteBuilder {
    fn new(source_dir: PathBuf, output_dir: PathBuf) -> Result<Self> {
        let liquid = ParserBuilder::with_stdlib().build()?;

        Ok(Self {
            source_dir,
            output_dir,
            config: SiteConfig::default(),
            layouts: HashMap::new(),
            posts: Vec::new(),
            pages: Vec::new(),
            liquid,
        })
    }

    fn build(&mut self) -> Result<()> {
        self.load_config()?;
        self.load_layouts()?;
        self.load_posts()?;
        self.load_pages()?;

        // Sort posts by date descending.
        self.posts.sort_by(|a, b| b.date.cmp(&a.date));

        self.render_posts()?;
        self.render_pages()?;
        self.copy_static_assets()?;
        self.generate_feed()?;
        self.generate_sitemap()?;

        Ok(())
    }

    fn load_config(&mut self) -> Result<()> {
        let config_path = self.source_dir.join("_config.yml");
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .context("Failed to read _config.yml")?;
            let cfg: ConfigFile = serde_yaml::from_str(&content)
                .context("Failed to parse _config.yml")?;

            self.config = SiteConfig {
                title: cfg.title.unwrap_or_default(),
                author: cfg.author.unwrap_or(Author {
                    name: None,
                    email: None,
                    github: None,
                }),
                url: cfg.url.unwrap_or_default(),
                permalink: cfg.permalink.unwrap_or_else(|| {
                    "/:categories/:year/:month/:day/:title".to_string()
                }),
                google_analytics: cfg.google_analytics,
            };
        }
        Ok(())
    }

    fn load_layouts(&mut self) -> Result<()> {
        let layouts_dir = self.source_dir.join("_layouts");
        if !layouts_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&layouts_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "html") {
                let name = path.file_stem().unwrap().to_string_lossy().to_string();
                let content = fs::read_to_string(&path)?;
                self.layouts.insert(name, content);
            }
        }
        Ok(())
    }

    fn load_posts(&mut self) -> Result<()> {
        // Load posts from multiple directories with their categories.
        let post_dirs = [
            ("_posts", vec![]),
            ("beer/_posts", vec!["beer".to_string()]),
            ("status-reports/_posts", vec!["status-reports".to_string()]),
        ];

        for (dir, categories) in post_dirs {
            let posts_dir = self.source_dir.join(dir);
            if posts_dir.exists() {
                self.load_posts_from_dir(&posts_dir, &categories)?;
            }
        }
        Ok(())
    }

    fn load_posts_from_dir(&mut self, dir: &Path, categories: &[String]) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "md" || e == "markdown") {
                match self.parse_post(&path, categories) {
                    Ok(post) => self.posts.push(post),
                    Err(e) => eprintln!("Warning: Failed to parse {}: {}", path.display(), e),
                }
            }
        }
        Ok(())
    }

    fn parse_post(&self, path: &Path, categories: &[String]) -> Result<Post> {
        let filename = path.file_stem().unwrap().to_string_lossy();

        // Parse date and slug from filename: YYYY-MM-DD-slug.md
        let (date, slug) = parse_post_filename(&filename)?;

        let content = fs::read_to_string(path)?;
        let (front_matter, body) = parse_front_matter(&content)?;

        // Title: from front matter, or derive from slug.
        let title = front_matter.title.unwrap_or_else(|| slug_to_title(&slug));

        let content_html = render_markdown(&body);
        let layout = front_matter.layout.unwrap_or_else(|| "post".to_string());
        let tags = front_matter.tags.unwrap_or_default();

        let url = build_permalink(
            &self.config.permalink,
            categories,
            &date,
            &slug,
        );

        Ok(Post {
            title,
            date,
            slug,
            categories: categories.to_vec(),
            tags,
            content_html,
            layout,
            url,
        })
    }

    fn load_pages(&mut self) -> Result<()> {
        // Explicit pages to load.
        let page_files = [
            "index.html",
            "blog/index.html",
            "beer/index.html",
            "status-reports/index.html",
            "resume.html",
            "rust-stuff.md",
            "writing.md",
            "worklog.md",
            "rust1.md",
            "styletest.md",
        ];

        for file in page_files {
            let path = self.source_dir.join(file);
            if path.exists() {
                match self.parse_page(&path, file) {
                    Ok(page) => self.pages.push(page),
                    Err(e) => eprintln!("Warning: Failed to parse {}: {}", file, e),
                }
            }
        }
        Ok(())
    }

    fn parse_page(&self, path: &Path, relative_path: &str) -> Result<Page> {
        let content = fs::read_to_string(path)?;
        let (front_matter, body) = parse_front_matter(&content)?;

        let is_html = path.extension().map_or(false, |e| e == "html");
        let content_html = if is_html {
            body
        } else {
            render_markdown(&body)
        };

        let layout = front_matter.layout.unwrap_or_else(|| "default".to_string());
        let title = front_matter.title.unwrap_or_default();

        // Output path: convert .md to .html.
        let output_path = if relative_path.ends_with(".md") {
            PathBuf::from(relative_path.replace(".md", ".html"))
        } else {
            PathBuf::from(relative_path)
        };

        let url = format!("/{}", output_path.display())
            .replace("/index.html", "/")
            .replace("index.html", "/");

        Ok(Page {
            title,
            output_path,
            content_html,
            layout,
            url,
            is_html,
        })
    }

    fn render_posts(&self) -> Result<()> {
        for post in &self.posts {
            let output_path = self.output_dir.join(url_to_path(&post.url));
            let html = self.render_post(post)?;
            write_file(&output_path, &html)?;
        }
        Ok(())
    }

    fn render_post(&self, post: &Post) -> Result<String> {
        let mut globals = self.build_site_globals();

        // Page variables.
        let mut page = Object::new();
        page.insert("title".into(), Value::scalar(post.title.clone()));
        page.insert("date".into(), Value::scalar(post.date.to_string()));
        page.insert("url".into(), Value::scalar(post.url.clone()));
        page.insert("content".into(), Value::scalar(post.content_html.clone()));

        let tags: Vec<Value> = post.tags.iter().map(|t| Value::scalar(t.clone())).collect();
        page.insert("tags".into(), Value::Array(tags));

        globals.insert("page".into(), Value::Object(page));
        globals.insert("content".into(), Value::scalar(post.content_html.clone()));

        self.render_with_layout(&post.layout, globals)
    }

    fn render_pages(&self) -> Result<()> {
        for page in &self.pages {
            let output_path = self.output_dir.join(&page.output_path);
            let html = self.render_page(page)?;
            write_file(&output_path, &html)?;
        }
        Ok(())
    }

    fn render_page(&self, page: &Page) -> Result<String> {
        let mut globals = self.build_site_globals();

        // Page variables.
        let mut page_obj = Object::new();
        page_obj.insert("title".into(), Value::scalar(page.title.clone()));
        page_obj.insert("url".into(), Value::scalar(page.url.clone()));

        globals.insert("page".into(), Value::Object(page_obj));

        // For pages that contain Liquid, we need to render the content first.
        let rendered_content = if page.is_html {
            // Render Liquid in the page content.
            let template = self.liquid.parse(&page.content_html)
                .context("Failed to parse page Liquid")?;
            template.render(&globals)?
        } else {
            page.content_html.clone()
        };

        globals.insert("content".into(), Value::scalar(rendered_content));

        self.render_with_layout(&page.layout, globals)
    }

    fn build_site_globals(&self) -> Object {
        let mut globals = Object::new();

        // Site variables.
        let mut site = Object::new();
        site.insert("title".into(), Value::scalar(self.config.title.clone()));
        site.insert("url".into(), Value::scalar(self.config.url.clone()));

        // Author.
        let mut author = Object::new();
        if let Some(name) = &self.config.author.name {
            author.insert("name".into(), Value::scalar(name.clone()));
        }
        if let Some(email) = &self.config.author.email {
            author.insert("email".into(), Value::scalar(email.clone()));
        }
        if let Some(github) = &self.config.author.github {
            author.insert("github".into(), Value::scalar(github.clone()));
        }
        site.insert("author".into(), Value::Object(author));

        // All posts.
        let posts_value = posts_to_liquid(&self.posts);
        site.insert("posts".into(), posts_value);

        // Categories map.
        let mut categories = Object::new();
        let mut cat_map: HashMap<String, Vec<&Post>> = HashMap::new();

        // Group posts by category.
        for post in &self.posts {
            if post.categories.is_empty() {
                cat_map.entry(String::new()).or_default().push(post);
            } else {
                for cat in &post.categories {
                    cat_map.entry(cat.clone()).or_default().push(post);
                }
            }
        }

        for (cat, cat_posts) in cat_map {
            let posts_val: Vec<Value> = cat_posts.iter().map(|p| post_to_liquid(p)).collect();
            categories.insert(KString::from_string(cat), Value::Array(posts_val));
        }
        site.insert("categories".into(), Value::Object(categories));

        globals.insert("site".into(), Value::Object(site));
        globals
    }

    fn render_with_layout(&self, layout_name: &str, mut globals: Object) -> Result<String> {
        let mut current_layout = layout_name.to_string();
        let mut content = globals.get("content")
            .and_then(|v| v.as_scalar())
            .map(|s| s.to_kstr().to_string())
            .unwrap_or_default();

        // Process layout chain.
        loop {
            let layout_content = self.layouts.get(&current_layout)
                .ok_or_else(|| anyhow!("Layout not found: {}", current_layout))?;

            // Check for parent layout in front matter.
            let (layout_fm, layout_body) = parse_front_matter(layout_content)?;

            globals.insert("content".into(), Value::scalar(content.clone()));

            let template = self.liquid.parse(&layout_body)
                .with_context(|| format!("Failed to parse layout: {}", current_layout))?;
            content = template.render(&globals)?;

            if let Some(parent) = layout_fm.layout {
                current_layout = parent;
            } else {
                break;
            }
        }

        Ok(content)
    }

    fn copy_static_assets(&self) -> Result<()> {
        let asset_dirs = [
            "css",
            "js",
            "images",
            "beer/images",
            "status-photos",
            "fontawesome",
            "lib",
            "tmp",
        ];

        for dir in asset_dirs {
            let src = self.source_dir.join(dir);
            let dst = self.output_dir.join(dir);
            if src.exists() {
                copy_dir_recursive(&src, &dst)?;
            }
        }
        Ok(())
    }

    fn generate_feed(&self) -> Result<()> {
        // Filter to main blog posts only (exclude beer and status-reports).
        let blog_posts: Vec<&Post> = self.posts
            .iter()
            .filter(|p| p.categories.is_empty())
            .take(20)
            .collect();

        let mut entries = String::new();
        for post in &blog_posts {
            let entry = format!(
                r#"  <entry>
    <title>{}</title>
    <link href="{}{}" rel="alternate" type="text/html"/>
    <id>{}{}</id>
    <published>{}T00:00:00Z</published>
    <updated>{}T00:00:00Z</updated>
    <content type="html"><![CDATA[{}]]></content>
  </entry>
"#,
                escape_xml(&post.title),
                self.config.url,
                post.url,
                self.config.url,
                post.url,
                post.date,
                post.date,
                post.content_html,
            );
            entries.push_str(&entry);
        }

        let feed = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>{}</title>
  <link href="{}/feed.xml" rel="self" type="application/atom+xml"/>
  <link href="{}" rel="alternate" type="text/html"/>
  <id>{}/</id>
  <updated>{}T00:00:00Z</updated>
{}
</feed>
"#,
            escape_xml(&self.config.title),
            self.config.url,
            self.config.url,
            self.config.url,
            blog_posts.first().map(|p| p.date.to_string()).unwrap_or_default(),
            entries,
        );

        let feed_path = self.output_dir.join("feed.xml");
        write_file(&feed_path, &feed)?;
        Ok(())
    }

    fn generate_sitemap(&self) -> Result<()> {
        let mut html = String::from(r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Sitemap</title>
  <style>
    body { font-family: sans-serif; max-width: 80ch; margin: 2em auto; padding: 0 1em; }
    h1 { border-bottom: 1px solid #ccc; padding-bottom: 0.5em; }
    h2 { margin-top: 2em; }
    ul { list-style: none; padding-left: 0; }
    li { margin: 0.3em 0; }
    a { color: #a040e0; }
    .date { color: #888; font-size: 0.9em; margin-right: 1em; }
  </style>
</head>
<body>
<h1>Sitemap</h1>
"#);

        // Pages section.
        html.push_str("<h2>Pages</h2>\n<ul>\n");
        for page in &self.pages {
            html.push_str(&format!(
                "  <li><a href=\"{}\">{}</a></li>\n",
                page.url,
                if page.title.is_empty() { &page.url } else { &page.title }
            ));
        }
        html.push_str("</ul>\n");

        // Blog posts section.
        let blog_posts: Vec<&Post> = self.posts
            .iter()
            .filter(|p| p.categories.is_empty())
            .collect();

        html.push_str("<h2>Blog Posts</h2>\n<ul>\n");
        for post in &blog_posts {
            html.push_str(&format!(
                "  <li><span class=\"date\">{}</span><a href=\"{}\">{}</a></li>\n",
                post.date, post.url, post.title
            ));
        }
        html.push_str("</ul>\n");

        // Beer reviews section.
        let beer_posts: Vec<&Post> = self.posts
            .iter()
            .filter(|p| p.categories.contains(&"beer".to_string()))
            .collect();

        html.push_str("<h2>Beer Reviews</h2>\n<ul>\n");
        for post in &beer_posts {
            html.push_str(&format!(
                "  <li><span class=\"date\">{}</span><a href=\"{}\">{}</a></li>\n",
                post.date, post.url, post.title
            ));
        }
        html.push_str("</ul>\n");

        // Status reports section.
        let status_posts: Vec<&Post> = self.posts
            .iter()
            .filter(|p| p.categories.contains(&"status-reports".to_string()))
            .collect();

        if !status_posts.is_empty() {
            html.push_str("<h2>Status Reports</h2>\n<ul>\n");
            for post in &status_posts {
                html.push_str(&format!(
                    "  <li><span class=\"date\">{}</span><a href=\"{}\">{}</a></li>\n",
                    post.date, post.url, post.title
                ));
            }
            html.push_str("</ul>\n");
        }

        html.push_str("</body>\n</html>\n");

        let sitemap_path = self.output_dir.join("sitemap.html");
        write_file(&sitemap_path, &html)?;
        Ok(())
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

fn parse_post_filename(filename: &str) -> Result<(NaiveDate, String)> {
    // Format: YYYY-MM-DD-slug
    if filename.len() < 11 {
        bail!("Filename too short: {}", filename);
    }

    let date_str = &filename[..10];
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .with_context(|| format!("Invalid date in filename: {}", filename))?;

    let slug = filename[11..].to_string();
    Ok((date, slug))
}

fn parse_front_matter(content: &str) -> Result<(FrontMatter, String)> {
    let content = content.trim_start();
    if !content.starts_with("---") {
        return Ok((FrontMatter::default(), content.to_string()));
    }

    let rest = &content[3..];
    let end = rest.find("\n---")
        .ok_or_else(|| anyhow!("Unclosed front matter"))?;

    let yaml = &rest[..end];
    let body = rest[end + 4..].trim_start().to_string();

    let fm: FrontMatter = serde_yaml::from_str(yaml)
        .unwrap_or_default();

    Ok((fm, body))
}

fn render_markdown(content: &str) -> String {
    let options = Options::all();
    let parser = MdParser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn slug_to_title(slug: &str) -> String {
    slug.replace('-', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn build_permalink(pattern: &str, categories: &[String], date: &NaiveDate, slug: &str) -> String {
    let year = date.format("%Y").to_string();
    let month = date.format("%m").to_string();
    let day = date.format("%d").to_string();

    let cat_path = if categories.is_empty() {
        String::new()
    } else {
        categories.join("/")
    };

    pattern
        .replace(":categories", &cat_path)
        .replace(":year", &year)
        .replace(":month", &month)
        .replace(":day", &day)
        .replace(":title", slug)
        .replace("//", "/")
}

fn url_to_path(url: &str) -> PathBuf {
    let url = url.trim_start_matches('/');
    if url.is_empty() || url.ends_with('/') {
        PathBuf::from(url).join("index.html")
    } else {
        PathBuf::from(format!("{}/index.html", url))
    }
}

fn post_to_liquid(post: &Post) -> Value {
    let mut obj = Object::new();
    obj.insert("title".into(), Value::scalar(post.title.clone()));
    obj.insert("date".into(), Value::scalar(post.date.to_string()));
    obj.insert("url".into(), Value::scalar(post.url.clone()));

    let tags: Vec<Value> = post.tags.iter().map(|t| Value::scalar(t.clone())).collect();
    obj.insert("tags".into(), Value::Array(tags));

    // Categories as an object with first/last properties for Jekyll compatibility.
    // Jekyll supports `array.first` syntax which liquid crate doesn't handle the same way.
    let mut cats_obj = Object::new();
    let first = post.categories.first().cloned().unwrap_or_default();
    let last = post.categories.last().cloned().unwrap_or_default();
    cats_obj.insert("first".into(), Value::scalar(first));
    cats_obj.insert("last".into(), Value::scalar(last));
    obj.insert("categories".into(), Value::Object(cats_obj));

    Value::Object(obj)
}

fn posts_to_liquid(posts: &[Post]) -> Value {
    Value::Array(posts.iter().map(post_to_liquid).collect())
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let src_path = entry.path();
        let rel_path = src_path.strip_prefix(src)?;
        let dst_path = dst.join(rel_path);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&dst_path)?;
        } else {
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

// =============================================================================
// Main
// =============================================================================

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("build");

    let source_dir = std::env::current_dir()?;
    let output_dir = source_dir.join("_site");

    match command {
        "build" => {
            println!("Building site...");
            let mut builder = SiteBuilder::new(source_dir, output_dir)?;
            builder.build()?;
            println!("Site built successfully to _site/");
        }
        "clean" => {
            if output_dir.exists() {
                fs::remove_dir_all(&output_dir)?;
                println!("Cleaned _site/");
            }
        }
        _ => {
            eprintln!("Usage: ssg [build|clean]");
            std::process::exit(1);
        }
    }

    Ok(())
}
