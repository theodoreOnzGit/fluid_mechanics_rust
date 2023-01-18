/// Contains structs or classes which
/// help you calculate pressure loss from mass 
/// flowrate and vice versa for pipes
/// with some fixed form losses
///
///
///
pub mod standard_pipe_calc;

/// Contains structs or classes which
/// help you calculate pressure loss from mass 
/// flowrate and vice versa for custom components
/// with custom friction factor and
/// form losses specified by the user
pub mod custom_component_calc;




/// This is a generic fluid component trait,
/// which specifies that fluid components in general
/// should have the following properties accessed
/// via get and set methods
///
/// ```rust
/// ```
///
pub mod fluid_component_trait;
pub use fluid_component_trait::*;


/// primitive tests and examples to use the FluidComponent 
/// Traits
pub mod tests_and_examples_simple;
pub use tests_and_examples_simple::*;


/// tests to show use of the traits in concurrency situations
/// with thread spawn and everything...
pub mod tests_and_examples_concurrency_multithreading;
pub use tests_and_examples_concurrency_multithreading::*;

