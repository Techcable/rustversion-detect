use crate::{Date, RustVersion};

pub enum ParseResult {
    Success(RustVersion),
    OopsClippy,
    Unrecognized,
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

fn parse_words(words: &mut dyn Iterator<Item = &str>) -> Option<RustVersion> {
    use crate::Channel::{Stable, Development, Beta, Nightly};

    let mut version_channel = words.next()?.split('-');
    let version = version_channel.next()?;
    let channel = version_channel.next();

    let mut digits = version.split('.');
    let major = digits.next()?.parse().ok()?;
    let minor = digits.next()?.parse().ok()?;
    let patch = digits.next().unwrap_or("0").parse().ok()?;

    let channel = match channel {
        None => Stable,
        Some("dev") => Development,
        Some(channel) if channel.starts_with("beta") => Beta,
        Some("nightly") => match words.next() {
            Some(hash) if hash.starts_with('(') => match words.next() {
                None if hash.ends_with(')') => Development,
                Some(date) if date.ends_with(')') => {
                    match date[..date.len() - 1].parse::<Date>() {
                        Ok(date) => Nightly { date },
                        Err(_) => return None,
                    }
                }
                None | Some(_) => return None,
            },
            Some(_) => return None,
            None => Development,
        },
        Some(_) => return None,
    };

    Some(RustVersion {
        major,
        minor,
        patch,
        channel,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        use crate::Channel::*;

        let cases = &[
            (
                "rustc 1.0.0 (a59de37e9 2015-05-13) (built 2015-05-14)",
                RustVersion {
                    major: 1,
                    minor: 0,
                    patch: 0,
                    channel: Stable,
                },
            ),
            (
                "rustc 1.18.0",
                RustVersion {
                    major: 1,
                    minor: 18,
                    patch: 0,
                    channel: Stable,
                },
            ),
            (
                "rustc 1.24.1 (d3ae9a9e0 2018-02-27)",
                RustVersion {
                    major: 1,
                    minor: 24,
                    patch: 1,
                    channel: Stable,
                },
            ),
            (
                "rustc 1.35.0-beta.3 (c13114dc8 2019-04-27)",
                RustVersion {
                    major: 1,
                    minor: 35,
                    patch: 0,
                    channel: Beta,
                },
            ),
            (
                "rustc 1.36.0-nightly (938d4ffe1 2019-04-27)",
                RustVersion {
                    major: 1,
                    minor: 36,
                    patch: 0,
                    channel: Nightly {
                        date: Date::new(
                            2019,
                            4,
                            27,
                        )
                    },
                },
            ),
            (
                "rustc 1.36.0-dev",
                RustVersion {
                    major: 1,
                    minor: 36,
                    patch: 0,
                    channel: Development,
                },
            ),
            (
                "rustc 1.36.0-nightly",
                RustVersion {
                    major: 1,
                    minor: 36,
                    patch: 0,
                    channel: Development,
                },
            ),
            (
                "warning: invalid logging spec 'warning', ignoring it
                 rustc 1.30.0-nightly (3bc2ca7e4 2018-09-20)",
                RustVersion {
                    major: 1,
                    minor: 30,
                    patch: 0,
                    channel: Nightly {
                        date: Date::new(
                            2018,
                            9,
                            20
                        ),
                    },
                },
            ),
            (
                "rustc 1.52.1-nightly (gentoo)",
                RustVersion {
                    major: 1,
                    minor: 52,
                    patch: 1,
                    channel: Development,
                },
            ),
        ];

        for (string, expected) in cases {
            match parse(string) {
                ParseResult::Success(version) => assert_eq!(version, *expected),
                ParseResult::OopsClippy | ParseResult::Unrecognized => {
                    panic!("unrecognized: {:?}", string);
                }
            }
        }
    }
}
