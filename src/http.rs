use std::{collections::HashMap, str::FromStr};

/// Represents an HTTP method.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

/// Provides functionality to convert a string into a `Method` enum.
/// Example: "GET" becomes `Method::GET`.
impl FromStr for Method {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(anyhow::anyhow!("No matching HTTP method")),
        }
    }
}

/// Provides functionality to convert a `Method` enum into a string.
/// Example: `Method::GET` becomes "GET".
impl ToString for Method {
    fn to_string(&self) -> String {
        match &self {
            Method::GET => "GET".to_string(),
            Method::HEAD => "HEAD".to_string(),
            Method::POST => "POST".to_string(),
            Method::PUT => "PUT".to_string(),
            Method::DELETE => "DELETE".to_string(),
            Method::CONNECT => "CONNECT".to_string(),
            Method::OPTIONS => "OPTIONS".to_string(),
            Method::TRACE => "TRACE".to_string(),
            Method::PATCH => "PATCH".to_string(),
        }
    }
}

/// Represents an HTTP version.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Version {
    V10,
    V11,
    V20,
    V30,
}

/// Provides functionality to convert a string into a `Version` enum.
/// Example: "HTTP/1.1" becomes `Version::V11`.
impl FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "HTTP/1.0" => Ok(Version::V10),
            "HTTP/1.1" => Ok(Version::V11),
            "HTTP/2.0" => Ok(Version::V20),
            "HTTP/3.0" => Ok(Version::V30),
            _ => Err(anyhow::anyhow!("No matching HTTP version")),
        }
    }
}

/// Provides functionality to convert a `Version` enum into a string.
/// Example: `Version::V11` becomes "HTTP/1.1".
impl ToString for Version {
    fn to_string(&self) -> String {
        match &self {
            Version::V10 => "HTTP/1.0".to_string(),
            Version::V11 => "HTTP/1.1".to_string(),
            Version::V20 => "HTTP/2.0".to_string(),
            Version::V30 => "HTTP/3.0".to_string(),
        }
    }
}

/// Represents common HTTP status codes.
#[derive(Debug)]
pub enum StatusCode {
    CODE100, // 100 Continue: Client should continue with the request.
    CODE102, // 102 Processing: Server is processing the request but no final response is available yet.
    CODE103, // 103 Early Hints: Used to preload resources before the final response is sent.
    CODE200, // 200 OK: The request was successful and the server returned the requested data.
    // CODE201, // 201 Created: The request was successful and a resource was created (e.g., after POST).
    CODE202, // 202 Accepted: The request was accepted for processing, but the processing is not complete.
    CODE204, // 204 No Content: The request was successful, but there is no content to send in the response.
    CODE205, // 205 Reset Content: Tells the client to reset the document view (e.g., after a form submission).
    CODE206, // 206 Partial Content: The server is delivering only part of the resource due to a range header in the request.
    CODE300, // 300 Multiple Choices: There are multiple options for the resource, and the user or agent must choose one.
    CODE301, // 301 Moved Permanently: The resource has been permanently moved to a new URL.
    CODE302, // 302 Found: The resource is temporarily available at a different URL.
    CODE303, // 303 See Other: The response to the request can be found under another URL using a GET method.
    CODE304, // 304 Not Modified: Indicates that the resource has not been modified since the last request (for caching purposes).
    CODE307, // 307 Temporary Redirect: The resource is temporarily available at a different URL, but the request method should not change.
    CODE308, // 308 Permanent Redirect: The resource has been permanently moved, and the client should use the new URL for future requests.
    CODE400, // 400 Bad Request: The server could not understand the request due to invalid syntax.
    CODE401, // 401 Unauthorized: The client must authenticate itself to get the requested response.
    CODE403, // 403 Forbidden: The client does not have access rights to the content.
    CODE404, // 404 Not Found: The server could not find the requested resource.
    CODE405, // 405 Method Not Allowed: The request method is known by the server but is not supported by the resource.
    CODE406, // 406 Not Acceptable: The server cannot produce a response that matches the criteria defined by the client's Accept headers.
    CODE408, // 408 Request Timeout: The server timed out waiting for the client to send a request.
    CODE409, // 418 I'm a Teapot: An April Fools' joke response code from the Hyper Text Coffee Pot Control Protocol.
    CODE500, // 500 Internal Server Error: The server encountered a situation it doesn't know how to handle.
    CODE501, // 501 Not Implemented: The request method is not supported by the server.
    CODE502, // 502 Bad Gateway: The server received an invalid response from the upstream server.
    CODE503, // 503 Service Unavailable: The server is not ready to handle the request (e.g., it's down for maintenance).
    CODE504, // 504 Gateway Timeout: The server, acting as a gateway, did not receive a timely response from the upstream server.
    CODE505, // 505 HTTP Version Not Supported: The server does not support the HTTP protocol version used in the request.
    CODE511, // 511 Network Authentication Required: The client needs to authenticate to gain network access (often used in captive portals).
}

/// Provides functionality to convert a `StatusCode` enum into a string.
/// Example: `StausCode::CODE100` becomes "100 Continue".
impl ToString for StatusCode {
    fn to_string(&self) -> String {
        match self {
            StatusCode::CODE100 => "100 Continue".to_string(),
            StatusCode::CODE102 => "102 Processing".to_string(),
            StatusCode::CODE103 => "103 Early Hints".to_string(),
            StatusCode::CODE200 => "200 OK".to_string(),
            //StatusCode::CODE201 => "201 Created".to_string(),
            StatusCode::CODE202 => "202 Accepted".to_string(),
            StatusCode::CODE204 => "204 No Content".to_string(),
            StatusCode::CODE205 => "205 Reset Content".to_string(),
            StatusCode::CODE206 => "206 Partial Content".to_string(),
            StatusCode::CODE300 => "300 Multiple Choices".to_string(),
            StatusCode::CODE301 => "301 Moved Permanently".to_string(),
            StatusCode::CODE302 => "302 Found".to_string(),
            StatusCode::CODE303 => "303 See Other".to_string(),
            StatusCode::CODE304 => "304 Not Modified".to_string(),
            StatusCode::CODE307 => "307 Temporary Redirect".to_string(),
            StatusCode::CODE308 => "308 Permanent Redirect".to_string(),
            StatusCode::CODE400 => "400 Bad Request".to_string(),
            StatusCode::CODE401 => "401 Unauthorized".to_string(),
            StatusCode::CODE403 => "403 Forbidden".to_string(),
            StatusCode::CODE404 => "404 Not Found".to_string(),
            StatusCode::CODE405 => "405 Method Not Allowed".to_string(),
            StatusCode::CODE406 => "406 Not Acceptable".to_string(),
            StatusCode::CODE408 => "408 Request Timeout".to_string(),
            StatusCode::CODE409 => "409 Conflict".to_string(),
            StatusCode::CODE500 => "500 Internal Server Error".to_string(),
            StatusCode::CODE501 => "501 Not Implemented".to_string(),
            StatusCode::CODE502 => "502 Bad Gateway".to_string(),
            StatusCode::CODE503 => "503 Service Unabailable".to_string(),
            StatusCode::CODE504 => "504 Gateway Timeout".to_string(),
            StatusCode::CODE505 => "505 HTTP Version Not Supported".to_string(),
            StatusCode::CODE511 => "511 Network Authentication Required".to_string(),
        }
    }
}

/// Represents an HTTP request with method, path, version, headers, and optional body.
#[derive(Debug)]
pub struct HTTPRequest {
    pub method: Method,
    pub path: String,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

/// Provides functionality to parse a raw HTTP request string into an `HTTPRequest` struct.
impl FromStr for HTTPRequest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Parse the request line (e.g., "GET /index.html HTTP/1.1")
        let request_line = lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid request"))?;
        let mut parts = request_line.split_whitespace();

        let method: Method = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing method"))?
            .parse()?;
        let path: String = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing path"))?
            .to_string();
        let version: Version = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing HTTP version"))?
            .parse()?;

        // Parse headers
        let mut headers = HashMap::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break; // Empty line marks the end of headers
            }
            let mut header_parts = line.splitn(2, ':');
            let header_name = header_parts.next().unwrap().trim().to_string();
            let header_value = header_parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Malformed header"))?
                .trim()
                .to_string();
            headers.insert(header_name, header_value);
        }

        // Parse body if there are remaining lines
        let body = if lines.clone().count() > 0 {
            Some(lines.collect::<Vec<&str>>().join("\n"))
        } else {
            None
        };

        Ok(HTTPRequest {
            method,
            path,
            version,
            headers,
            body,
        })
    }
}

impl HTTPRequest {
    /// Retrieve a specific header from the HTTP request.
    ///
    /// # Arguments
    ///
    /// * `request` - The HTTP request to retrieve from.
    /// * `header` - The name of the header to get.
    ///
    /// # Returns
    ///
    /// Returns a `Option` containing the `String` value or None.
    pub fn get_header(request: &HTTPRequest, header: &str) -> Option<String> {
        match request.headers.get(header) {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }

    /// Retrieve the cookies from the HTTP request.
    ///
    /// # Arguments
    ///
    /// * `request` - The HTTP request to retrieve from.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `HashMap<String, String>` map containing the cookies or an error.
    pub fn get_cookies(request: &HTTPRequest) -> anyhow::Result<HashMap<String, String>> {
        let mut cookies = HashMap::new();
        let data = match HTTPRequest::get_header(request, "Cookie") {
            Some(data) => data,
            None => return Err(anyhow::anyhow!("No Cookie header found")),
        };

        for cookie in data.split(";") {
            let mut split = cookie.split("=");

            let name = split.next().unwrap();
            let value = split.next().unwrap();

            cookies.insert(name.to_string(), value.to_string());
        }

        Ok(cookies)
    }

    /// Retrieve a specific cookie by its name.
    ///
    /// # Arguments
    ///
    /// * `request` - The HTTP request to retrieve from.
    /// * `cookie` - The name of the cookie to get.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `String` value of the cookie or an error.
    pub fn get_cookie(request: &HTTPRequest, cookie: &str) -> anyhow::Result<String> {
        let cookies = HTTPRequest::get_cookies(request)?;

        match cookies.get(cookie) {
            Some(cookie) => Ok(cookie.to_string()),
            None => {
                return Err(anyhow::anyhow!(
                    "No cookie with this name ({}) exists",
                    cookie
                ));
            }
        }
    }
}

/// Represents an HTTP response with version, status code, headers, and optional body.
#[derive(Debug)]
pub struct HTTPResponse {
    pub version: Version,
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

/// Provides functionality to parse a `HTTPRequest` struct into an HTTP response string.
impl ToString for HTTPResponse {
    fn to_string(&self) -> String {
        format!(
            "{} {}\n{}",
            self.version.to_string(),
            self.status_code.to_string(),
            self.headers
                .iter()
                .map(|(k, v)| { format!("{}: {}", k, v) })
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
