use derive_more::Deref;
use std::collections::HashMap;
use std::error::Error;

/// Template String, new-type wrapper
#[derive(Debug, Deref)]
pub struct Template(pub String);

/// Loads the embedded template as the Template new-type
///
/// # Returns
///
/// - `Result<Template, Box<dyn Error>>` - Template string wrapper, propagating error value.
///
pub fn load_template(template: &str) -> Result<Template, Box<dyn Error>> {
    Ok(Template(template.to_string()))
}

/// Renders a given template with its paramaters swapped out
///
/// # Arguments
///
/// - `template` (`HTML`) - An HTML string containing template keys.
///   These keys should having two curly braces and a space around them. Something like:
///   ```template
///       <h2>Record your attendance for {{ DATE }} </h2>s
///   ```
/// - `map` (`HashMap<String, String>`) - Map of template keys to template values. So something like
///   ```template
///       <h2>Record your attendance for {{ DATE }} </h2>
///   ```
///   with the map record of <"DATE", "10-05-2026"> would render was
///   ```template
///       <h2>Record your attendance for 10-05-2026 </h2>
///   ```
/// # Returns
///
/// - `HTML` - HTML with all templates filled out
///
pub fn render_template(template: Template, map: HashMap<String, String>) -> Template {
    let mut update_me = template.clone();
    map.iter().for_each(|(key, value)| {
        // { is escaped with {, so to replace {{ key }} you need {{{{ {key} }}}}
        update_me = update_me.replace(&format!("{{{{ {key} }}}}"), value);
    });

    Template(update_me)
}
