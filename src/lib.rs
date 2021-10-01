//entrypoint to the program
pub mod entrypoint;

//program API, (de)serializing instruction data
pub mod instruction;

//program logic
pub mod processor;

//program objects, (de)serializing state
pub mod state;

//program specific errors
pub mod error;
