#[macro_export]
macro_rules! ua_parser {
    ($parser:ident, $($name:ident($($pattern:expr),+ $(,)?) => |$($param:ident),* $(,)?| $body:block),+ $(,)?) => {
        use std::collections::HashMap;

        use regex::{Regex, RegexSet};

        struct $parser {
            regex: RegexSet,
            regexes: HashMap<String, Regex>,
            callbacks: HashMap<String, String>,
        }

        impl $parser {
            fn new() -> $parser {
                let patterns = vec![$($($pattern),+),*];
                let regex = RegexSet::new(patterns).unwrap();
                let mut regexes = HashMap::new();
                let mut callbacks = HashMap::new();

                $($(
                    regexes.insert($pattern.to_string(), Regex::new($pattern).unwrap());
                    callbacks.insert($pattern.to_string(), String::from(stringify!($name)))
                );+);*;

                $parser{regex, regexes, callbacks}
            }

            fn parse(&self, input: &str) -> Result<Option<UserAgent>, UserAgentParseError> {
                for match_ in self.regex.matches(input).iter() {
                    let re = &self.regex.patterns()[match_];
                    let func_name = &self.callbacks[re];
                    let caps = self.regexes[re].captures(input).unwrap();

                    let parsed = match func_name.as_ref() {
                        $(stringify!($name) => {
                            $parser::$name($(&caps[stringify!($param).trim_start_matches('_')]),*)
                        }),*,
                        _ => {
                            panic!("Invalid callback");
                        }
                    };

                    match parsed {
                        IOption::Some(ua) => return Ok(Some(ua)),
                        IOption::Ignored => return Ok(None),
                        IOption::None => continue,
                    };
                }

                error!("invalid user agent"; "user_agent" => input);

                Err(UserAgentParseError{ua: input.to_string()})
            }

            $(fn $name($($param: &str),*) -> IOption<UserAgent> $body)*
        }
    };
}

#[macro_export]
macro_rules! installer {
    ($name:expr) => {
        Some(Installer {
            name: Some($name.to_string()),
            ..Default::default()
        })
    };

    ($name:expr, $version:expr) => {
        Some(Installer {
            name: Some($name.to_string()),
            version: Some($version.to_string()),
        })
    };
}

#[macro_export]
macro_rules! implementation {
    ($($name:ident : $value:expr),* $(,)?) => {
        Some(Implementation { $($name: $value),*, ..Default::default() })
    };
}

#[macro_export]
macro_rules! distro {
    ($($name:ident : $value:expr),* $(,)?) => {
        Some(Distro { $($name: $value),*, ..Default::default() })
    };
}

#[macro_export]
macro_rules! system {
    ($($name:ident : $value:expr),* $(,)?) => {
        Some(System { $($name: $value),*, ..Default::default() })
    };
}

#[macro_export]
macro_rules! user_agent {
    ($($name:ident : $value:expr),* $(,)?) => {
        IOption::Some(UserAgent { $($name: $value),*, ..Default::default() })
    };
}

#[macro_export]
macro_rules! lower {
    ($name:ident) => {
        $name.to_string().to_lowercase().as_ref()
    };
}

#[macro_export]
macro_rules! without_unknown {
    ($name:ident) => {
        match $name.to_string().to_lowercase().as_ref() {
            "unknown" => None,
            _ => Some($name.to_string()),
        };
    };
}