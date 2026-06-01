use anyhow::{Result, Context};

pub fn getconf(args: &str) -> Result<()> {
    let result = args.split_once("=").map(|(_, result)| result).context("Failed to get the full parameter, check the syntax")?.split_whitespace();
    return result;
}

pub fn gettext(args: &str) -> Result<()> {
    let result = args.split_once("=").map(|(_, result)| result).context("Failed to get the text displayed, check the syntax")?.split_once("<").map(|(_, result)| result).context("Check the syntax checking text")?.split_once(">").map(|(result, _)| result).context("Please check the syntax")?.split_whitespace();
    return result;
}

pub fn getimagepath(args: &str) -> Result<()> {
    let result = args.split_once("=").map(|(_, result)| result).context("Failed to get the image path, check the syntax")?.split_whitespace();
    return result;
}


pub fn loadconf() -> Result<()> {
    let conf = fs::read_to_string("/etc/distro-welcomer.conf")?
    return conf;
}