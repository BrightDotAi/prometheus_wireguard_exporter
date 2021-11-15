use crate::metrics::MetricAttributeOptions;

#[derive(Debug, Clone)]
pub(crate) struct Options {
    pub verbose: bool,
    pub prepend_sudo: bool,
    pub extract_names_config_files: Option<Vec<String>>,
    pub interfaces: Option<Vec<String>>,
    pub metric_attributes: MetricAttributeOptions,
}

impl Options {
    pub fn from_claps(matches: &clap::ArgMatches<'_>) -> Options {
        let handshake_timeout_seconds = matches
            .value_of("handshake_timeout_seconds")
            .map(|timeout| timeout.parse::<u64>().unwrap());

        let options = Options {
            verbose: matches
                .value_of("verbose")
                .map(|e| {
                    e.to_lowercase()
                        .parse()
                        .expect("cannot parse verbose as a bool")
                })
                .unwrap_or_default(),
            prepend_sudo: matches
                .value_of("prepend_sudo")
                .map(|e| {
                    e.to_lowercase()
                        .parse()
                        .expect("cannot parse prepend_sudo as a bool")
                })
                .unwrap_or_default(),
            metric_attributes: MetricAttributeOptions {
                split_allowed_ips: matches
                    .value_of("separate_allowed_ips")
                    .map(|e| {
                        e.to_lowercase()
                            .parse()
                            .expect("cannot parse separate_allowed_ips as a bool")
                    })
                    .unwrap_or_default(),
                export_remote_ip_and_port: matches
                    .value_of("export_remote_ip_and_port")
                    .map(|e| {
                        e.to_lowercase()
                            .parse()
                            .expect("cannot parse export_remote_ip_and_port as a bool")
                    })
                    .unwrap_or_default(),
                handshake_timeout_seconds,
            },

            extract_names_config_files: matches
                .values_of("extract_names_config_files")
                .map(|e| e.into_iter().map(|e| e.to_owned()).collect()),
            interfaces: matches
                .values_of("interfaces")
                .map(|e| e.into_iter().map(|a| a.to_owned()).collect()),
        };

        options
    }
}
