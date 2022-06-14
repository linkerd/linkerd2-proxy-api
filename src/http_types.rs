use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
};
use thiserror::Error;

include!("gen/io.linkerd.proxy.http_types.rs");

/// Indicates an HTTP Method could not be decoded.
#[derive(Clone, Debug, Error)]
#[error("invalid HTTP method")]
pub struct InvalidMethod;

/// Indicates a URI Scheme could not be decoded.
#[derive(Debug, Error)]
pub enum InvalidScheme {
    #[error("scheme must have a type")]
    MissingType,

    #[error("unexpected registered scheme: {0}")]
    UnexpectedRegistered(i32),

    #[error("invalid unregistered scheme: {0}")]
    InvalidUnregstered(#[from] http::uri::InvalidUri),
}

// === impl scheme::Type ===

impl TryInto<Cow<'static, str>> for &'_ scheme::Type {
    type Error = InvalidScheme;

    fn try_into(self) -> Result<Cow<'static, str>, Self::Error> {
        use scheme::*;

        match *self {
            Type::Registered(reg) => {
                if reg == Registered::Http.into() {
                    Ok(Cow::Borrowed("http"))
                } else if reg == Registered::Https.into() {
                    Ok(Cow::Borrowed("https"))
                } else {
                    Err(InvalidScheme::UnexpectedRegistered(reg))
                }
            }
            Type::Unregistered(ref s) => Ok(Cow::Owned(s.clone())),
        }
    }
}

// === impl http::HttpMethod ===

impl TryFrom<HttpMethod> for http::Method {
    type Error = InvalidMethod;

    fn try_from(proto: HttpMethod) -> Result<http::Method, Self::Error> {
        proto.r#type.ok_or(InvalidMethod)?.try_into()
    }
}

impl TryFrom<http_method::Type> for http::Method {
    type Error = InvalidMethod;

    fn try_from(proto: http_method::Type) -> Result<http::Method, Self::Error> {
        use http_method::*;

        match proto {
            Type::Registered(reg) => {
                if reg == Registered::Get.into() {
                    Ok(http::Method::GET)
                } else if reg == Registered::Post.into() {
                    Ok(http::Method::POST)
                } else if reg == Registered::Put.into() {
                    Ok(http::Method::PUT)
                } else if reg == Registered::Delete.into() {
                    Ok(http::Method::DELETE)
                } else if reg == Registered::Patch.into() {
                    Ok(http::Method::PATCH)
                } else if reg == Registered::Options.into() {
                    Ok(http::Method::OPTIONS)
                } else if reg == Registered::Connect.into() {
                    Ok(http::Method::CONNECT)
                } else if reg == Registered::Head.into() {
                    Ok(http::Method::HEAD)
                } else if reg == Registered::Trace.into() {
                    Ok(http::Method::TRACE)
                } else {
                    Err(InvalidMethod)
                }
            }
            Type::Unregistered(ref m) => m.parse().map_err(|_| InvalidMethod),
        }
    }
}

impl From<http::Method> for http_method::Type {
    fn from(m: http::Method) -> Self {
        use http_method::*;

        match m {
            http::Method::GET => Type::Registered(Registered::Get.into()),
            http::Method::POST => Type::Registered(Registered::Post.into()),
            http::Method::PUT => Type::Registered(Registered::Put.into()),
            http::Method::DELETE => Type::Registered(Registered::Delete.into()),
            http::Method::PATCH => Type::Registered(Registered::Patch.into()),
            http::Method::HEAD => Type::Registered(Registered::Head.into()),
            http::Method::OPTIONS => Type::Registered(Registered::Options.into()),
            http::Method::CONNECT => Type::Registered(Registered::Connect.into()),
            http::Method::TRACE => Type::Registered(Registered::Trace.into()),
            ref method => Type::Unregistered(method.as_str().into()),
        }
    }
}

impl From<http::Method> for HttpMethod {
    fn from(m: http::Method) -> Self {
        HttpMethod {
            r#type: Some(m.into()),
        }
    }
}

// === impl Scheme ===

impl<'a> From<&'a http::uri::Scheme> for Scheme {
    fn from(scheme: &'a http::uri::Scheme) -> Self {
        scheme.as_ref().into()
    }
}

impl<'a> From<&'a str> for scheme::Type {
    fn from(s: &'a str) -> Self {
        use scheme::*;

        match s {
            "http" => Type::Registered(Registered::Http.into()),
            "https" => Type::Registered(Registered::Https.into()),
            s => Type::Unregistered(s.into()),
        }
    }
}

impl<'a> From<&'a str> for Scheme {
    fn from(s: &'a str) -> Self {
        Scheme {
            r#type: Some(s.into()),
        }
    }
}

impl TryFrom<Scheme> for http::uri::Scheme {
    type Error = InvalidScheme;

    fn try_from(s: Scheme) -> Result<Self, Self::Error> {
        match s.r#type.ok_or(InvalidScheme::MissingType)? {
            scheme::Type::Registered(typ) => {
                if typ == scheme::Registered::Http.into() {
                    Ok(http::uri::Scheme::HTTP)
                } else if typ == scheme::Registered::Https.into() {
                    Ok(http::uri::Scheme::HTTPS)
                } else {
                    Err(InvalidScheme::UnexpectedRegistered(typ))
                }
            }
            scheme::Type::Unregistered(typ) => Ok(typ.parse()?),
        }
    }
}

#[cfg(feature = "arbitrary")]
mod arbitary {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for HttpMethod {
        fn arbitrary(g: &mut Gen) -> Self {
            HttpMethod {
                r#type: Arbitrary::arbitrary(g),
            }
        }
    }

    impl Arbitrary for http_method::Type {
        fn arbitrary(g: &mut Gen) -> Self {
            match u16::arbitrary(g) % 9 {
                8 => http_method::Type::Unregistered(String::arbitrary(g)),
                n => http_method::Type::Registered(n.into()),
            }
        }
    }

    impl Arbitrary for Scheme {
        fn arbitrary(g: &mut Gen) -> Self {
            Scheme {
                r#type: Arbitrary::arbitrary(g),
            }
        }
    }

    impl Arbitrary for scheme::Type {
        fn arbitrary(g: &mut Gen) -> Self {
            match u16::arbitrary(g) % 3 {
                3 => scheme::Type::Unregistered(String::arbitrary(g)),
                n => scheme::Type::Registered(n.into()),
            }
        }
    }
}
