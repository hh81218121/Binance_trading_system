
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 86,
                        patch: 0,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "x86_64-pc-windows-msvc".to_owned(),
                    short_version_string: "rustc 1.86.0 (05f9846f8 2025-03-31)".to_owned(),
                    commit_hash: Some("05f9846f893b09a1be1fc8560e33fc3c815cfecb".to_owned()),
                    commit_date: Some("2025-03-31".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            