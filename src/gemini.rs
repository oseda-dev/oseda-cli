use std::error::Error;

use gemini_client_api::gemini::{ask::Gemini, types::sessions::Session};

use crate::net::await_future;

pub fn get_gemini_response(prompt: &str) -> Result<String, Box<dyn Error>> {


    let mut session = Session::new(1); 
    let ai = Gemini::new(
        std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set"),
        "gemini-2.5-flash",
        None,
    );

    let response = await_future(ai.ask(session.ask("Hello from my rust CLI")))?;
    let text = response.get_chat().get_text_no_think("\n");


    Ok(text)

}
