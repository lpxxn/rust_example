// If you need to access an element of phrases/greetings.rs from outside the module, 
// you have to import the greetings module as a public module.
pub mod greetings;

pub use self::greetings::hello2; // Re-export `greetings::hello` to phrases
/*
self keyword is used to refer same module, 
while super keyword is used to refer parent module. 
Also super keyword can be used to access root functions from inside a module.
*/