use derive_more::Deref;
use std::collections::HashMap;
use std::error::Error;

/// HTML String, new-type wrapper
#[derive(Debug, Deref)]
pub struct Html(pub String);

// going to need this in the binary eventually anyway
const FRONTEND_TEMPLATE: &str = include_str!("../templates/index.html");

/// Loads the embedded HTML template as the HTML new-type
///
/// # Returns
///
/// - `Result<HTML, Box<dyn Error>>` - HTML string wrapper, propogating error value.
///
pub fn load_template() -> Result<Html, Box<dyn Error>> {
    // let mut buf: String = String::new();

    // let mut file = File::open(path)?;
    // let contents = file.read_to_string(&mut buf)?;

    // Ok(HTML(contents.to_string()))

    Ok(Html(FRONTEND_TEMPLATE.to_string()))
}

/// Renders a given template with its paramaters swapped out
///
/// # Arguments
///
/// - `html` (`HTML`) - An HTML string containing template keys.
///   These keys should having two curly braces and a space around them. Something like:
///   ```html
///       <h2>Record your attendance for {{ DATE }} </h2>s
///   ```
/// - `map` (`HashMap<String, String>`) - Map of template keys to template values. So something like
///   ```html
///       <h2>Record your attendance for {{ DATE }} </h2>
///   ```
///   with the map record of <"DATE", "10-05-2026"> would render was
///   ```html
///       <h2>Record your attendance for 10-05-2026 </h2>
///   ```
/// # Returns
///
/// - `HTML` - HTML with all templates filled out
///
pub fn render_template(html: Html, map: HashMap<String, String>) -> Html {
    let mut update_me = html.clone();
    map.iter().for_each(|(key, value)| {
        // { is escaped with {, so to replace {{ key }} you need {{{{ {key} }}}}
        update_me = update_me.replace(&format!("{{{{ {key} }}}}"), value);
    });

    Html(update_me)
}
