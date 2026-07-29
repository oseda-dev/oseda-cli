use std::{error::Error, path::Path};

use gemini_client_api::{gemini::{ask::Gemini, types::{request::InlineData, sessions::Session}}, utils::mime};

use crate::net::await_future;


const CONVERSION_PROMPT: &str = include_str!("conversion_prompt.md");


pub fn get_gemini_response(pdf_path: &str) -> Result<String, Box<dyn Error>> {




    //afaik, this is bascially just a session builder
    let mut session = Session::new(1); 
    let ai = Gemini::new(
        std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set"),
        // this should prolly be configurable
        "gemini-2.5-flash-lite",
        None,
    );

    let data = await_future(
        InlineData::from_path(pdf_path, mime::APPLICATION_PDF)
    )?;

    println!("data was: {:?}", data);

    session.ask(CONVERSION_PROMPT).ask(data);


    let response = await_future(
        ai.ask(&mut session)
    )?;

    let text = response.get_chat().get_text_no_think("\n");


    Ok(text)

}
