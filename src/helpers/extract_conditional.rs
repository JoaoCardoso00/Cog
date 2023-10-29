use crate::frontend::parser::ast::{ASTStatementKind, ConditionalStatement};

impl ASTStatementKind {
    pub fn extract_conditional(&self) -> Option<&ConditionalStatement> {
        if let ASTStatementKind::ConditionalStatement(ref conditional) = self {
            Some(conditional)
        } else {
            None
        }
    }

    pub fn extract_conditional_mut(&mut self) -> Option<&mut ConditionalStatement> {
        if let ASTStatementKind::ConditionalStatement(ref mut conditional) = self {
            Some(conditional)
        } else {
            None
        }
    }
}
