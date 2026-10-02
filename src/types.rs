use std::fmt;
use std::str::FromStr;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Trace,
    Connect,
}
impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Trace => "TRACE",
            HttpMethod::Connect => "CONNECT",
        };
        write!(f, "{}", s)
    }
}
impl FromStr for HttpMethod {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let upper = s.to_uppercase();
        if upper == "GET" {
            return Ok(HttpMethod::Get);
        }
        if upper == "POST" {
            return Ok(HttpMethod::Post);
        }
        if upper == "PUT" {
            return Ok(HttpMethod::Put);
        }
        if upper == "DELETE" {
            return Ok(HttpMethod::Delete);
        }
        if upper == "PATCH" {
            return Ok(HttpMethod::Patch);
        }
        if upper == "HEAD" {
            return Ok(HttpMethod::Head);
        }
        if upper == "OPTIONS" {
            return Ok(HttpMethod::Options);
        }
        if upper == "TRACE" {
            return Ok(HttpMethod::Trace);
        }
        if upper == "CONNECT" {
            return Ok(HttpMethod::Connect);
        }
        Err(format!("未知的 HTTP 方法: {}", upper))
    }
}
impl TryFrom<&str> for HttpMethod {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}
use std::collections::HashMap;
/// Unified global Request type
#[derive(Debug, Clone)]
pub struct Request {
    /// HTTP method
    pub method: HttpMethod,
    /// Request path, e.g. /users/1
    pub path: String,
    /// Query parameters, e.g. ?page=1&size=10
    pub query: HashMap<String, String>,
    /// Request headers (keys are lowercase)
    pub headers: HashMap<String, String>,
    /// Request body
    pub body: Vec<u8>,
}
impl Request {
    /// Create an empty request
    pub fn new(method: HttpMethod, path: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
            query: HashMap::new(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
    /// Builder-style: set body
    pub fn with_body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }
    /// Builder-style: set header
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into().to_lowercase(), value.into());
        self
    }
    /// Get a header (case-insensitive)
    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers.get(&key.to_lowercase())
    }
    /// Parse body as UTF-8 string
    pub fn body_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.body)
    }
}
/// Unified global Response type
#[derive(Debug, Clone)]
pub struct Response {
    /// Status code, e.g. 200 / 404
    pub status: u16,
    /// Response headers (keys are lowercase)
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: Vec<u8>,
}
impl Response {
    /// Create an empty response
    pub fn new(status: u16) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
    /// 200 OK with a text body
    pub fn ok(body: impl Into<Vec<u8>>) -> Self {
        Self::new(200).with_body(body)
    }
    /// 404 Not Found
    pub fn not_found() -> Self {
        Self::new(404).with_body("Not Found")
    }
    /// 500 Internal Server Error
    pub fn internal_error(msg: impl Into<Vec<u8>>) -> Self {
        Self::new(500).with_body(msg)
    }
    /// Builder-style: set body
    pub fn with_body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }
    /// Builder-style: set header
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into().to_lowercase(), value.into());
        self
    }
    /// Get a header (case-insensitive)
    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers.get(&key.to_lowercase())
    }
    /// Parse body as UTF-8 string
    pub fn body_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.body)
    }
}
