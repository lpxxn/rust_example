// If you need to access an element of phrases/greetings.rs from outside the module, 
// you have to import the greetings module as a public module.
pub mod greetings;
pub fn hello1() {
    greetings::hello2();
}