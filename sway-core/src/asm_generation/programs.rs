mod r#abstract;
mod allocated;
mod r#final;

use super::fuel::{
    abstract_instruction_set::AbstractInstructionSet,
    allocated_abstract_instruction_set::AllocatedAbstractInstructionSet, data_section::DataSection,
    register_sequencer::RegisterSequencer,
};

use crate::{
    asm_lang::{allocated_ops::AllocatedOp, Label},
    decl_engine::DeclRefFunction,
    ExperimentalFlags,
};

type SelectorOpt = Option<[u8; 4]>;
type FnName = String;
type ImmOffset = u64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgramKind {
    Contract,
    Library,
    Predicate,
    Script,
}

/// An [AbstractProgram] represents code generated by the compilation from IR, with virtual registers
/// and abstract control flow.
///
/// Use `AbstractProgram::to_allocated_program()` to perform register allocation.
///
pub(super) struct AbstractProgram {
    kind: ProgramKind,
    data_section: DataSection,
    entries: Vec<AbstractEntry>,
    non_entries: Vec<AbstractInstructionSet>,
    reg_seqr: RegisterSequencer,
    experimental: ExperimentalFlags,
}

/// The entry point of an abstract program.
pub(super) struct AbstractEntry {
    pub(super) selector: SelectorOpt,
    pub(super) label: Label,
    pub(super) ops: AbstractInstructionSet,
    pub(super) name: FnName,
    pub(super) test_decl_ref: Option<DeclRefFunction>,
}

/// An [AllocatedProgram] represents code which has allocated registers but still has abstract
/// control flow.
pub(super) struct AllocatedProgram {
    kind: ProgramKind,
    data_section: DataSection,
    prologue: AllocatedAbstractInstructionSet,
    functions: Vec<AllocatedAbstractInstructionSet>,
    entries: Vec<(SelectorOpt, Label, FnName, Option<DeclRefFunction>)>,
}

/// A [FinalProgram] represents code which may be serialized to VM bytecode.
pub(super) enum FinalProgram {
    Fuel {
        kind: ProgramKind,
        data_section: DataSection,
        ops: Vec<AllocatedOp>,
        entries: Vec<(SelectorOpt, ImmOffset, FnName, Option<DeclRefFunction>)>,
    },
    Evm {
        ops: Vec<etk_asm::ops::AbstractOp>,
        abi: Vec<ethabi::operation::Operation>,
    },
    MidenVM {
        ops: Vec<crate::asm_generation::DirectOp>,
    },
}
