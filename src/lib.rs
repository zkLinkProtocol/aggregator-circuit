use franklin_crypto::bellman::plonk::better_better_cs::gates::selector_optimized_with_d_next::SelectorOptimizedWidth4MainGateWithDNext as MainGate;
use franklin_crypto::bellman::plonk::better_better_cs::proof::Proof;
use sync_vm::recursion::aggregation::MainGateParametrizedCircuitWithNonlinearityAndLookups as MainCircuit;

mod final_aggregation;
mod oracle_aggregation;
mod padding;

pub use sync_vm::utils; // for cs_derive proc macro
pub use franklin_crypto::bellman; // for cs_derive proc macro
pub use franklin_crypto;
pub use sync_vm as advanced_components;

pub use final_aggregation::*;
pub use oracle_aggregation::*;

pub type UniformCircuit<E> = MainCircuit<E, MainGate>;
pub type UniformProof<E> = Proof<E, UniformCircuit<E>>;
