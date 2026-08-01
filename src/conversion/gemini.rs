use std::{error::Error, path::Path};

use gemini_client_api::{gemini::{ask::Gemini, types::{request::InlineData, sessions::Session}}, utils::mime};

use crate::net::await_future;


const CONVERSION_PROMPT: &str = include_str!("conversion_prompt.md");

pub fn convert_pdf(pdf_path: &str, extracted_images: &[String]) -> Result<String, Box<dyn Error>> {
    let mut session = Session::new(1); 
    let ai = Gemini::new(
        std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set"),
        // todo make this configurable
        "gemini-2.5-flash-lite",
        None,
    );

    let catalog_text = if extracted_images.is_empty() {
        "No extracted images available.".to_string()
    } else {
        extracted_images
            .iter()
            .map(|img| format!("- {}", img))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let prompt = CONVERSION_PROMPT.replace("{{IMAGE_CATALOG}}", &catalog_text);

    let data = await_future(
        InlineData::from_path(pdf_path, mime::APPLICATION_PDF)
    )?;

    // todo put in loop, so the user doesnt have to redo everything
    // when the (hoepfully) chosen model is busy 
    session.ask(prompt).ask(data);

    let response = await_future(ai.ask(&mut session))?;
    let text = response.get_chat().get_text_no_think("\n");

    Ok(text)
}