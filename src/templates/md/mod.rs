use std::{collections::HashMap, error::Error, fs};

use crate::templates::{Renderer, RendererError, render_template};


pub const MD_VITE_CONFIG_JS: &str = include_str!("static/vite.config.js");
pub const MD_INDEX_HTML: &str = include_str!("static/index.html");
pub const MD_MAIN_JS: &str = include_str!("static/main.js");
pub const MD_SLIDES: &str = include_str!("static/slides.md");
pub const MD_CUSTOM_CSS: &str = include_str!("static/custom.css");
pub const MD_FERRIS: &[u8] = include_bytes!("static/ferris.png");
pub const MD_GITIGNORE: &str = include_str!("static/.gitignore");
pub const MD_FAVICON: &[u8] = include_bytes!("static/favicon.png");

pub struct MarkdownRenderer {
    pub params: HashMap<String, String>,
}

impl Renderer for MarkdownRenderer {
    fn write_to_fs(&self, target_dir: &str) -> Result<(), RendererError> {
        fs::write(
            format!("{}/vite.config.js", target_dir),
            render_template(MD_VITE_CONFIG_JS, &self.params),
        )?;

        fs::write(
            format!("{}/index.html", target_dir),
            render_template(MD_INDEX_HTML, &self.params),
        )?;
        fs::write(
            format!("{}/.gitignore", target_dir),
            render_template(MD_GITIGNORE, &self.params),
        )?;

        std::fs::create_dir_all(format!("{}/src", target_dir))?;
        fs::write(
            format!("{}/src/main.js", target_dir),
            render_template(MD_MAIN_JS, &self.params),
        )?;

        std::fs::create_dir_all(format!("{}/slides", target_dir))?;
        fs::write(
            format!("{}/slides/slides.md", target_dir),
            render_template(MD_SLIDES, &self.params),
        )?;

        std::fs::create_dir_all(format!("{}/css", target_dir))?;
        fs::write(
            format!("{}/css/custom.css", target_dir),
            render_template(MD_CUSTOM_CSS, &self.params),
        )?;


        // binary files do not need templated
        std::fs::create_dir_all(format!("{}/public", target_dir))?;
        fs::write(
            format!("{}/public/ferris.png", target_dir),
            MD_FERRIS,
        )?;
        fs::write(
            format!("{}/public/favicon.png", target_dir),
            MD_FAVICON,
        )?;
        
        Ok(()) 
    }

}
