use std::collections::HashMap;

use crate::{
    http::{HTTPRequest, Method, Version},
    HandlerFunction,
};

/// A struct to manage HTTP routes and their associated handler functions.
#[derive(Debug)]
pub struct Router {
    routes: HashMap<(Method, String, Version), HandlerFunction>,
}

impl Router {
    /// Creates a new instance of `Router`.
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    /// Adds a new route to the router.
    ///
    /// This method allows you to register a handler function for a specific HTTP method,
    /// path, and version.
    ///
    /// # Parameters
    /// - `method`: The HTTP method (e.g., GET, POST) for the route.
    /// - `path`: A string slice that represents the path for the route.
    /// - `version`: The HTTP version associated with the route.
    /// - `handler`: A `HandlerFunction` that will be invoked when the route is matched.
    pub fn add_route(
        &mut self,
        method: Method,
        path: &str,
        version: Version,
        handler: HandlerFunction,
    ) {
        self.routes
            .insert((method, path.to_string(), version), handler);
    }

    /// Retrieves the handler function for a given HTTP request.
    ///
    /// This method checks if there is a route that matches the request's method,
    /// path, and version. If a matching route exists, it returns a reference to
    /// the associated `HandlerFunction`.
    ///
    /// # Parameters
    /// - `request`: A reference to an `HTTPRequest` that contains the method, path, and version.
    ///
    /// # Returns
    /// An `Option<&HandlerFunction>`, which will be `Some(handler)` if a matching route is found,
    /// or `None` if there is no match.
    pub fn route(&self, request: &HTTPRequest) -> Option<&HandlerFunction> {
        self.routes.get(&(
            request.method.clone(),
            request.path.clone(),
            request.version.clone(),
        ))
    }
}
