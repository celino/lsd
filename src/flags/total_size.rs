//! This module defines the [TotalSize] flag. To set it up from [ArgMatches], a [Yaml] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use yaml_rust::Yaml;

/// The flag showing whether to show the total size for directories.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct TotalSize(pub bool);

impl Configurable<Self> for TotalSize {
    /// Get a potential `TotalSize` value from [ArgMatches].
    ///
    /// If the "total-size" argument is passed, this returns a `TotalSize` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("total-size") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `TotalSize` value from a [Config].
    ///
    /// If the Config's [Yaml] contains the [Boolean](Yaml::Boolean) value pointed to by
    /// "total-size", this returns its value as the value of the `TotalSize`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(yaml) = &config.yaml {
            match &yaml["total-size"] {
                Yaml::BadValue => None,
                Yaml::Boolean(value) => Some(Self(*value)),
                _ => {
                    config.print_wrong_type_warning("total-size", "boolean");
                    None
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::TotalSize;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, TotalSize::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--total-size"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(TotalSize(true)), TotalSize::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, TotalSize::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, TotalSize::from_config(&Config::with_yaml(yaml)));
    }

    #[test]
    fn test_from_config_true() {
        let yaml_string = "total-size: true";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(TotalSize(true)),
            TotalSize::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_false() {
        let yaml_string = "total-size: false";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(TotalSize(false)),
            TotalSize::from_config(&Config::with_yaml(yaml))
        );
    }
}
