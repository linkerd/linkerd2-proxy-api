#![cfg(feature = "arbitrary")]

use std::boxed::Box;

use quickcheck::*;
use rand::Rng;

use super::http_types::*;
use super::net::*;
use super::tap::*;

impl Arbitrary for ObserveRequest {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        ObserveRequest {
            limit: g.gen(),
            r#match: Arbitrary::arbitrary(g),
        }
    }
}

impl Arbitrary for observe_request::Match {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        observe_request::Match {
            r#match: Arbitrary::arbitrary(g),
        }
    }
}

impl Arbitrary for observe_request::r#match::Match {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        match g.gen::<u32>() % 6 {
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
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        observe_request::r#match::Seq {
            matches: Arbitrary::arbitrary(g),
        }
    }

    fn shrink(&self) -> Box<Iterator<Item = Self>> {
        Box::new(self.matches.shrink().map(|matches| {
            observe_request::r#match::Seq {
                matches,
            }
        }))
    }
}

impl Arbitrary for observe_request::r#match::Label {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        observe_request::r#match::Label {
            key: Arbitrary::arbitrary(g),
            value: Arbitrary::arbitrary(g),
        }
    }
}

impl Arbitrary for observe_request::r#match::Tcp {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        observe_request::r#match::Tcp {
            r#match: Arbitrary::arbitrary(g),
        }
    }
}

impl Arbitrary for observe_request::r#match::tcp::Match {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        use self::observe_request::r#match::tcp;

        if g.gen::<bool>() {
            tcp::Match::Netmask(Arbitrary::arbitrary(g))
        } else {
            tcp::Match::Ports(Arbitrary::arbitrary(g))
        }
    }
}

impl Arbitrary for observe_request::r#match::tcp::PortRange {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        observe_request::r#match::tcp::PortRange {
            min: g.gen(),
            max: g.gen(),
        }
    }
}

impl Arbitrary for observe_request::r#match::tcp::Netmask {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        let ip: Option<IpAddress> = Arbitrary::arbitrary(g);
        let mask = match ip.as_ref().and_then(|a| a.ip.as_ref()) {
            Some(&ip_address::Ip::Ipv4(_)) => g.gen::<u32>() % 32 + 1,
            Some(&ip_address::Ip::Ipv6(_)) => g.gen::<u32>() % 128 + 1,
            None => 0u32,
        };
        observe_request::r#match::tcp::Netmask {
            ip,
            mask,
        }
    }
}

impl Arbitrary for observe_request::r#match::Http {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        observe_request::r#match::Http {
            r#match: Arbitrary::arbitrary(g),
        }
    }
}

impl Arbitrary for observe_request::r#match::http::Match {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        use self::observe_request::r#match::http;

        match g.gen::<u32>() % 4 {
            0 => http::Match::Scheme(Scheme::arbitrary(g)),
            1 => http::Match::Method(HttpMethod::arbitrary(g)),
            2 => http::Match::Authority(http::StringMatch::arbitrary(g)),
            3 => http::Match::Path(http::StringMatch::arbitrary(g)),
            _ => unreachable!(),
        }
    }
}

impl Arbitrary for observe_request::r#match::http::StringMatch {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        observe_request::r#match::http::StringMatch {
            r#match: Arbitrary::arbitrary(g),
        }
    }
}

impl Arbitrary for observe_request::r#match::http::string_match::Match {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        use self::observe_request::r#match::http::string_match;

        match g.gen::<u32>() % 2 {
            0 => string_match::Match::Exact(String::arbitrary(g)),
            1 => string_match::Match::Prefix(String::arbitrary(g)),
            _ => unreachable!(),
        }
    }
}

impl Arbitrary for IpAddress {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        IpAddress {
            ip: Arbitrary::arbitrary(g),
        }
    }
}

impl Arbitrary for ip_address::Ip {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        if g.gen::<bool>() {
            ip_address::Ip::Ipv4(g.gen())
        } else {
            ip_address::Ip::Ipv6(IPv6::arbitrary(g))
        }
    }
}

impl Arbitrary for IPv6 {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        IPv6 {
            first: g.gen(),
            last: g.gen(),
        }
    }
}

impl Arbitrary for HttpMethod {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        HttpMethod {
            r#type: Arbitrary::arbitrary(g),
        }
    }
}

impl Arbitrary for http_method::Type {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        match g.gen::<u16>() % 9 {
            8 => http_method::Type::Unregistered(String::arbitrary(g)),
            n => http_method::Type::Registered(i32::from(n).into()),
        }
    }
}

impl Arbitrary for Scheme {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        Scheme {
            r#type: Arbitrary::arbitrary(g),
        }
    }
}

impl Arbitrary for scheme::Type {
    fn arbitrary<G: Gen + Rng>(g: &mut G) -> Self {
        match g.gen::<u16>() % 3 {
            3 => scheme::Type::Unregistered(String::arbitrary(g)),
            n => scheme::Type::Registered(i32::from(n).into()),
        }
    }
}
