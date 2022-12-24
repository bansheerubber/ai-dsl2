pub mod compile;
pub mod control_flow;
pub mod function;
pub mod loops;
pub mod math;
pub mod variable_assignment;
pub mod variable_declaration;

pub use compile::{ CompilationContext, compile_pair, compile_pairs, };
pub use loops::for_loop::ForLoop;
pub use function::Function;
pub use control_flow::if_statement::IfStatement;
pub use math::Math;
pub use control_flow::return_statement::Return;
pub use variable_assignment::VariableAssignment;
pub use variable_declaration::VariableDeclaration;
pub use loops::while_loop::WhileLoop;
