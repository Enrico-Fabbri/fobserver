use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// A struct to manage a collection of arguments, allowing for dynamic typing and thread-safe access.
#[derive(Debug)]
pub struct Args {
    args: HashMap<String, Arc<RwLock<dyn Any + Send + Sync>>>,
}

impl Args {
    /// Creates a new instance of `Args`.
    pub fn new() -> Self {
        Args {
            args: HashMap::new(),
        }
    }

    /// Adds an argument to the `Args` collection.
    ///
    /// This method allows you to insert a new argument, identified by a string `name`,
    /// along with its value wrapped in `Arc<RwLock<dyn Any + Send + Sync>>`.
    ///
    /// # Parameters
    /// - `name`: A string slice that represents the name of the argument.
    /// - `arg`: An `Arc<RwLock<dyn Any + Send + Sync>>` containing the argument value.
    ///
    /// # Returns
    /// A mutable reference to `self` to allow for method chaining.
    pub fn add_arg(&mut self, name: &str, arg: Arc<RwLock<dyn Any + Send + Sync>>) -> &mut Self {
        self.args.insert(name.to_string(), arg);

        self
    }

    /// Retrieves an argument from the `Args` collection by its name.
    ///
    /// This method returns an `Option<Arc<RwLock<dyn Any + Send + Sync>>>`.
    /// If the argument with the specified `name` exists, it returns `Some(arg)`.
    /// Otherwise, it returns `None`.
    ///
    /// # Parameters
    /// - `name`: A string slice that represents the name of the argument to retrieve.
    ///
    /// # Returns
    /// An `Option` containing the argument if it exists, or `None` if it does not.
    pub fn arg(&self, name: &str) -> Option<Arc<RwLock<dyn Any + Send + Sync>>> {
        self.args.get(name).cloned()
    }
}
