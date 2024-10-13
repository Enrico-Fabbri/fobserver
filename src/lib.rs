use std::{
    cmp::min,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock},
    thread,
};

use args::Args;
use http::{HTTPRequest, HTTPResponse};
use router::Router;

pub mod args;
pub mod http;
pub mod router;

/// A type alias for a function that handles HTTP requests.
///
/// This function takes an `HTTPRequest` and an `Arc<RwLock<Args>>` as parameters,
/// and returns a result containing an `HTTPResponse`. It is used to define
/// the structure of handler functions that can process HTTP requests.
///
/// # Parameters
/// - `request`: An instance of `HTTPRequest` that represents the incoming HTTP request.
/// - `args`: An `Arc<RwLock<Args>>` containing additional arguments that can be accessed
///   and modified safely across multiple threads.
///
/// # Returns
/// An `anyhow::Result<HTTPResponse>`, which indicates the success or failure of the
/// request handling. On success, it returns an `HTTPResponse`, and on failure, it
/// returns an error wrapped in `anyhow::Error`.
pub type HandlerFunction = fn(HTTPRequest, Arc<RwLock<Args>>) -> anyhow::Result<HTTPResponse>;

/// Represents an HTTP server that listens for incoming connections.
///
/// # Example
///
/// This example demonstrates how to create a server that counts the number of
/// GET requests received at the root endpoint:
///
/// ```
/// use std::sync::{Arc, RwLock};
/// use your_library::{Args, HTTPRequest, HTTPResponse, Router, Server};
///
/// struct Counter {
///     value: usize,
/// }
///
/// impl Counter {
///     pub fn new() -> Self {
///         Counter { value: 0 }
///     }
///
///     pub fn add(&mut self) {
///         self.value += 1;
///     }
/// }
///
/// fn main() -> anyhow::Result<()> {
///     let mut args = Args::new();
///     args.add_arg("counter", Arc::new(RwLock::new(Counter::new())));
///
///     let mut router = Router::new();
///     fn handler(_: HTTPRequest, args: Arc<RwLock<Args>>) -> anyhow::Result<HTTPResponse> {
///         let args = args.read().unwrap();
///
///         let binding = args.arg("counter").unwrap();
///         let mut l = binding.write().unwrap();
///         let counter = l.downcast_mut::<Counter>().unwrap();
///
///         counter.add();
///
///         let response = HTTPResponse {
///             version: http::Version::V11,
///             status_code: http::StatusCode::CODE200,
///             headers: HashMap::new(),
///             body: Some(format!("Counter value: {}", counter.value).into()),
///         };
///
///         Ok(response)
///     }
///
///     router.add_route(http::Method::GET, "/", http::Version::V11, handler);
///
///     let mut server = Server::new("192.168.1.131:30303", router, args)?;
///     server.start()
/// }
/// ```
pub struct Server {
    listener: TcpListener,
    router: Arc<RwLock<Router>>,
    args: Arc<RwLock<Args>>,
}

impl Server {
    /// Creates a new `Server` instance bound to the specified address.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to bind the server to.
    /// * `router` - The router for handling HTTP requests.
    /// * `args` - Arguments to be shared across handlers.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Server` instance or an error.
    pub fn new(addr: &str, router: Router, args: Args) -> anyhow::Result<Self> {
        let listener = TcpListener::bind(addr)?;

        Ok(Server {
            listener,
            router: Arc::new(RwLock::new(router)),
            args: Arc::new(RwLock::new(args)),
        })
    }

    /// Reads an HTTP request from the given TCP stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - The TCP stream to read from.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `HTTPRequest` or an error.
    fn read_request(mut stream: &TcpStream) -> anyhow::Result<HTTPRequest> {
        let mut buffer = [0; 4096];
        let mut dim = 0;

        loop {
            let len = stream.read(&mut buffer)?;

            dim += len;

            if len < 4096 {
                break;
            }
        }

        String::from_utf8_lossy(&buffer)[..dim].to_string().parse()
    }

    /// Writes an HTTP response to the given TCP stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - The TCP stream to write the response to.
    /// * `response` - The `HTTPResponse` to be sent.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure.
    fn write_response(mut stream: &TcpStream, mut response: HTTPResponse) -> anyhow::Result<()> {
        response
            .headers
            .insert("Transfer-Encoding".to_string(), "chunked".to_string());

        // response
        // .headers
        // .insert("Keep-Alive".to_string(), "true".to_string());

        let body = response.body.clone();

        stream.write_all(format!("{}\n\n", response.to_string()).as_bytes())?;

        if let Some(bytes) = body {
            let mut start = 0;

            while start < bytes.len() {
                let len = min(4096, bytes.len() - start);

                stream.write_all(format!("{:X}\r\n", len).as_bytes())?;
                stream.write_all(&bytes[start..start + len])?;
                stream.write_all(b"\r\n")?;

                start += len;
            }
        }

        stream.write_all(b"0\r\n\r\n")?;

        Ok(())
    }

    /// Starts the server, listening for incoming requests.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure.
    pub fn start(&mut self) -> anyhow::Result<()> {
        log::info!("Server started on {}", self.listener.local_addr()?);

        for stream in self.listener.incoming() {
            let router = self.router.clone();
            let args = self.args.clone();

            match stream {
                Ok(stream) => {
                    thread::spawn(move || -> anyhow::Result<()> {
                        // read request
                        let request = Server::read_request(&stream)?;
                        // Find path
                        let response = match router.read() {
                            Ok(router) => match router.route(&request) {
                                Some(function) => function(request, args)?,
                                None => {
                                    return Err(anyhow::anyhow!(
                                        "Error: No associated functions to request -> {:#?}",
                                        request
                                    ))
                                }
                            },
                            Err(err) => return Err(anyhow::anyhow!("Error: {}", err)),
                        };
                        // send response and close connection
                        Server::write_response(&stream, response)
                    });
                }
                Err(err) => return Err(anyhow::anyhow!("Error: {}", err)),
            }
        }

        Ok(())
    }
}
