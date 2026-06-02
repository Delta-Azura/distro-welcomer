use anyhow::{Result, Context};
use std::fs;

pub fn getconf(args: &str) -> Result<(String)> {
    let result = args.split_once("=").map(|(_, result)| result).context("Failed to get the full parameter, check the syntax")?.split_whitespace().next().context("Failed to get the first element")?.to_string();
    return Ok(result.to_string());
}

pub fn gettext(args: &str) -> Result<(String)> {
    let result = args.split_once("=").map(|(_, result)| result).context("Failed to get the text displayed, check the syntax")?.split_once("<").map(|(_, result)| result).context("Check the syntax checking text")?.split_once(">").map(|(result, _)| result).context("Please check the syntax")?.split_whitespace().next().context("Failed to get the first element")?.to_string();
    return Ok(result.to_string());
}

pub fn getimagepath(args: &str) -> Result<(String)> {
    let result = args.split_once("=").map(|(_, result)| result).context("Failed to get the image path, check the syntax")?.split_whitespace().next().context("Failed to get the first element")?.to_string();
    return Ok(result.to_string());
}


pub fn loadconf() -> Result<(String, String, String, String)> {
    let conf = fs::read_to_string("/etc/distro-welcomer.conf")?;
    let website = String::new();
    let documentation = String::new();
    let community = String::new();
    let image = String::new();
    if conf.contains("website=") {
        let website = conf.lines().find(|l| l.starts_with("website=")).context("Failed to find the website parameter")?;
        let website = getconf(&website).context("Failed to get the website parameter")?;
    }
    if conf.contains("documentation=") {
        let documentation = conf.lines().find(|l| l.starts_with("documentation=")).context("Failed to find the documentation parameter")?;
        let documentation = getconf(&documentation).context("Failed to get the documentation parameter")?;
    }
    if conf.contains("community=") {
        let community = conf.lines().find(|l| l.starts_with("community=")).context("Failed to find the community parameter")?;
        let community = getconf(&community).context("Failed to get the community parameter")?;
    }
    if conf.contains("image=") {
        let image = conf.lines().find(|l| l.starts_with("image=")).context("Failed to find the image parameter")?;
        let image = getimagepath(&image).context("Failed to get the image parameter")?;
    }
    println!("{} {} {} {}", website, documentation, community, image);
    return Ok((website, documentation, community, image));
}