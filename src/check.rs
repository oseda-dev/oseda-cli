use std::error::Error;

use clap::Args;

#[derive(Args, Debug)]
pub struct CheckOptions {
    #[arg(long, default_value_t = 8080)]
    port: u16,
}

pub fn check(opts: CheckOptions) -> Result<(), Box<dyn Error>> {
    // need to check
    // config
    // - Verify it exists
    // - match author name to github name
    // - verify categories
    // - Title doesnt have spaces
    //  - (maybe an additional filter here?, scunthorpe filtering lol?)
    //
    // run
    // - should be able to sucessfully run project
    // - `oseda run &` and then like ping the host?
    // - but if we are pinging the host, we may need to make that configurable
    // - because the end user may wanna test on a diff port
    //
    //

    Ok(())
}
