use self::Channel::*;

pub enum ParseResult {
    Success(Version),
    OopsClippy,
    Unrecognized,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[allow(dead_code)] // used by Debug
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub channel: Channel,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[allow(dead_code)] // used by Debug
pub enum Channel {
    Stable,
    Beta,
    Nightly(Date),
    Dev,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[allow(dead_code)] // used by Debug
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

pub fn parse(string: &str) -> ParseResult {
    let last_line = string.lines().last().unwrap_or(string);
    let mut words = last_line.trim().split(' ');

    match words.next() {
        Some("rustc") => {}
        Some(word) if word.starts_with("clippy") => return ParseResult::OopsClippy,
        Some(_) | None => return ParseResult::Unrecognized,
    }

    parse_words(&mut words).map_or(ParseResult::Unrecognized, ParseResult::Success)
}

fn parse_words(words: &mut dyn Iterator<Item = &str>) -> Option<Version> {
    let mut version_channel = words.next()?.split('-');
    let version = version_channel.next()?;
    let channel = version_channel.next();

    let mut digits = version.split('.');
    let major = digits.next()?.parse().ok()?;
    let minor = digits.next()?.parse().ok()?;
    let patch = digits.next().unwrap_or("0").parse().ok()?;

    let channel = match channel {
        None => Stable,
        Some("dev") => Dev,
        Some(channel) if channel.starts_with("beta") => Beta,
        Some("nightly") => match words.next() {
            Some(hash) if hash.starts_with('(') => match words.next() {
                None if hash.ends_with(')') => Dev,
                Some(date) if date.ends_with(')') => {
                    let mut date = date[..date.len() - 1].split('-');
                    let year = date.next()?.parse().ok()?;
                    let month = date.next()?.parse().ok()?;
                    let day = date.next()?.parse().ok()?;
                    match date.next() {
                        None => Nightly(Date { year, month, day }),
                        Some(_) => return None,
                    }
                }
                None | Some(_) => return None,
            },
            Some(_) => return None,
            None => Dev,
        },
        Some(_) => return None,
    };

    Some(Version {
        major,
        minor,
        patch,
        channel,
    })
}
