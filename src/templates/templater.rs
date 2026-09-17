use std::collections::HashMap;

use crate::templates::{
    html::HtmlRenderer, md::MarkdownRenderer, CLITemplate, Renderer, RendererError,
};

pub struct Templater {
    template_type: CLITemplate,
    params: HashMap<String, String>,
}

impl Templater {
    pub fn new(template_type: CLITemplate, params: HashMap<String, String>) -> Self {
        Self {
            template_type,
            params,
        }
    }

    pub fn update_template(&self, text: &str) -> String {
        let mut update_me = String::from(text);
        self.params.iter().for_each(|(key, value)| {
            // { is escaped with {, so to replace {{ key }} you need {{{{ {key} }}}}
            update_me = update_me.replace(&format!("{{{{ {key} }}}}"), value);
        });

        update_me
    }
}

impl Renderer for Templater {
    fn write_to_fs(&self, target_dir: &str) -> Result<(), RendererError> {
        match self.template_type {
            CLITemplate::HTML => {
                let renderer = HtmlRenderer {
                    params: self.params.clone(),
                };
                renderer.write_to_fs(target_dir)
            }
            CLITemplate::Markdown => {
                let renderer = MarkdownRenderer {
                    params: self.params.clone(),
                };
                renderer.write_to_fs(target_dir)
            }
        }
    }
}
