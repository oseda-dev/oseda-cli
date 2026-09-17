use std::{collections::HashMap, fs};

use crate::templates::{Renderer, RendererError, render_template};

pub const HTML_VITE_CONFIG_JS: &str = include_str!("static/vite.config.js");
pub const HTML_INDEX_HTML: &str = include_str!("static/index.html");
pub const HTML_MAIN_JS: &str = include_str!("static/main.js");
pub const HTML_SLIDES: &str = include_str!("static/slides.html");
pub const HTML_CUSTOM_CSS: &str = include_str!("static/custom.css");
pub const HTML_FERRIS: &[u8] = include_bytes!("static/ferris.png");
pub const HTML_GITIGNORE: &str = include_str!("static/.gitignore");
pub const HTML_FAVICON: &[u8] = include_bytes!("static/favicon.png");

pub struct HtmlRenderer {
    pub params: HashMap<String, String>,
}

impl Renderer for HtmlRenderer {
    fn write_to_fs(&self, target_dir: &str) -> Result<(), RendererError> {
        fs::write(
            format!("{}/vite.config.js", target_dir),
            render_template(HTML_VITE_CONFIG_JS, &self.params),
        )?;
        fs::write(
            format!("{}/index.html", target_dir),
            render_template(HTML_INDEX_HTML, &self.params),
        )?;
        fs::write(
            format!("{}/.gitignore", target_dir),
            render_template(HTML_GITIGNORE, &self.params),
        )?;

        std::fs::create_dir_all(format!("{}/src", target_dir))?;
        fs::write(
            format!("{}/src/main.js", target_dir),
            render_template(HTML_MAIN_JS, &self.params),
        )?;

        std::fs::create_dir_all(format!("{}/slides", target_dir))?;
        fs::write(
            format!("{}/slides/slides.html", target_dir),
            render_template(HTML_SLIDES, &self.params),
        )?;

        std::fs::create_dir_all(format!("{}/css", target_dir))?;
        fs::write(
            format!("{}/css/custom.css", target_dir),
            render_template(HTML_CUSTOM_CSS, &self.params),
        )?;

        std::fs::create_dir_all(format!("{}/public", target_dir))?;
        fs::write(
            format!("{}/public/ferris.png", target_dir),
            HTML_FERRIS,
        )?;
        fs::write(
            format!("{}/public/favicon.png", target_dir),
            HTML_FAVICON,
        )?;


        Ok(())
    }
}
