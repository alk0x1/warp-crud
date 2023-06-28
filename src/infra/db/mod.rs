// Declare the folder module
pub mod connection;

// Export the public API of the folder module
pub use connection::establish_connection;
pub use connection::perform_query;