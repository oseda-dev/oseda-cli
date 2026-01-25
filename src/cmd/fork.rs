use std::error::Error;


pub fn fork() -> Result<(), Box<dyn Error>> {

    let fork_url = "https://github.com/oseda-dev/oseda-lib/fork".to_owned();

    open::that(fork_url.clone()).map_err(|_|{
        format!("Please visit {fork_url} in a browser and fork the oseda-lib repository")
    })?;

    Ok(())
}
