include!("gen/io.linkerd.proxy.tap.rs");

// === impl Eos ===

impl From<h2::Reason> for Eos {
    fn from(reason: h2::Reason) -> Self {
        let end = eos::End::ResetErrorCode(reason.into());
        Eos { end: Some(end) }
    }
}

impl Eos {
    pub fn from_grpc_status(code: u32) -> Self {
        let end = eos::End::GrpcStatusCode(code);
        Eos { end: Some(end) }
    }
}

#[cfg(feature = "arbitrary")]
mod arbitary {
    use super::*;
    use crate::{http_types::*, net::*};
    use quickcheck::*;

    impl Arbitrary for ObserveRequest {
        fn arbitrary(g: &mut Gen) -> Self {
            ObserveRequest {
                limit: Arbitrary::arbitrary(g),
                r#match: Arbitrary::arbitrary(g),
                extract: Arbitrary::arbitrary(g),
            }
        }
    }

    impl Arbitrary for observe_request::Match {
        fn arbitrary(g: &mut Gen) -> Self {
            observe_request::Match {
                r#match: Arbitrary::arbitrary(g),
            }
        }
    }

    impl Arbitrary for observe_request::r#match::Match {
        fn arbitrary(g: &mut Gen) -> Self {
            match u32::arbitrary(g) % 6 {
                0 => observe_request::r#match::Match::All(Arbitrary::arbitrary(g)),
                1 => observe_request::r#match::Match::Any(Arbitrary::arbitrary(g)),
                2 => observe_request::r#match::Match::Not(Box::new(Arbitrary::arbitrary(g))),
                3 => observe_request::r#match::Match::Source(Arbitrary::arbitrary(g)),
                4 => observe_request::r#match::Match::Destination(Arbitrary::arbitrary(g)),
                5 => observe_request::r#match::Match::Http(Arbitrary::arbitrary(g)),
                _ => unreachable!(),
            }
        }
    }

    impl Arbitrary for observe_request::r#match::Seq {
        fn arbitrary(g: &mut Gen) -> Self {
            observe_request::r#match::Seq {
                matches: Arbitrary::arbitrary(g),
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(
                self.matches
                    .shrink()
                    .map(|matches| observe_request::r#match::Seq { matches }),
            )
        }
    }

    impl Arbitrary for observe_request::r#match::Label {
        fn arbitrary(g: &mut Gen) -> Self {
            observe_request::r#match::Label {
                key: Arbitrary::arbitrary(g),
                value: Arbitrary::arbitrary(g),
            }
        }
    }

    impl Arbitrary for observe_request::r#match::Tcp {
        fn arbitrary(g: &mut Gen) -> Self {
            observe_request::r#match::Tcp {
                r#match: Arbitrary::arbitrary(g),
            }
        }
    }

    impl Arbitrary for observe_request::r#match::tcp::Match {
        fn arbitrary(g: &mut Gen) -> Self {
            use self::observe_request::r#match::tcp;

            if bool::arbitrary(g) {
                tcp::Match::Netmask(Arbitrary::arbitrary(g))
            } else {
                tcp::Match::Ports(Arbitrary::arbitrary(g))
            }
        }
    }

    impl Arbitrary for observe_request::r#match::tcp::PortRange {
        fn arbitrary(g: &mut Gen) -> Self {
            observe_request::r#match::tcp::PortRange {
                min: Arbitrary::arbitrary(g),
                max: Arbitrary::arbitrary(g),
            }
        }
    }

    impl Arbitrary for observe_request::r#match::tcp::Netmask {
        fn arbitrary(g: &mut Gen) -> Self {
            let ip: Option<IpAddress> = Arbitrary::arbitrary(g);
            let mask = match ip.as_ref().and_then(|a| a.ip.as_ref()) {
                Some(&ip_address::Ip::Ipv4(_)) => u32::arbitrary(g) % 32 + 1,
                Some(&ip_address::Ip::Ipv6(_)) => u32::arbitrary(g) % 128 + 1,
                None => 0u32,
            };
            observe_request::r#match::tcp::Netmask { ip, mask }
        }
    }

    impl Arbitrary for observe_request::r#match::Http {
        fn arbitrary(g: &mut Gen) -> Self {
            observe_request::r#match::Http {
                r#match: Arbitrary::arbitrary(g),
            }
        }
    }

    impl Arbitrary for observe_request::r#match::http::Match {
        fn arbitrary(g: &mut Gen) -> Self {
            use self::observe_request::r#match::http;

            match u32::arbitrary(g) % 4 {
                0 => http::Match::Scheme(Scheme::arbitrary(g)),
                1 => http::Match::Method(HttpMethod::arbitrary(g)),
                2 => http::Match::Authority(http::StringMatch::arbitrary(g)),
                3 => http::Match::Path(http::StringMatch::arbitrary(g)),
                _ => unreachable!(),
            }
        }
    }

    impl Arbitrary for observe_request::r#match::http::StringMatch {
        fn arbitrary(g: &mut Gen) -> Self {
            observe_request::r#match::http::StringMatch {
                r#match: Arbitrary::arbitrary(g),
            }
        }
    }

    impl Arbitrary for observe_request::r#match::http::string_match::Match {
        fn arbitrary(g: &mut Gen) -> Self {
            use self::observe_request::r#match::http::string_match;

            match u16::arbitrary(g) % 2 {
                0 => string_match::Match::Exact(String::arbitrary(g)),
                1 => string_match::Match::Prefix(String::arbitrary(g)),
                _ => unreachable!(),
            }
        }
    }

    impl Arbitrary for observe_request::Extract {
        fn arbitrary(g: &mut Gen) -> Self {
            observe_request::Extract {
                extract: Arbitrary::arbitrary(g),
            }
        }
    }

    impl Arbitrary for observe_request::extract::Extract {
        fn arbitrary(g: &mut Gen) -> Self {
            observe_request::extract::Extract::Http(Arbitrary::arbitrary(g))
        }
    }

    impl Arbitrary for observe_request::extract::Http {
        fn arbitrary(g: &mut Gen) -> Self {
            observe_request::extract::Http {
                extract: Arbitrary::arbitrary(g),
            }
        }
    }

    impl Arbitrary for observe_request::extract::http::Extract {
        fn arbitrary(g: &mut Gen) -> Self {
            observe_request::extract::http::Extract::Headers(Arbitrary::arbitrary(g))
        }
    }

    impl Arbitrary for observe_request::extract::http::Headers {
        fn arbitrary(_: &mut Gen) -> Self {
            Self {}
        }
    }
}
