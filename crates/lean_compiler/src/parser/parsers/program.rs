use super::function::FunctionParser;
use super::literal::ConstantDeclarationParser;
use super::{Parse, ParseContext, ParsedConstant};
use crate::{
    lang::Program,
    parser::{
        error::{ParseResult, SemanticError},
        grammar::{ParsePair, Rule},
        parsers::next_inner_pair,
    },
};
use std::collections::BTreeMap;

/// Parser for complete programs.
pub struct ProgramParser;

impl Parse<(Program, BTreeMap<usize, String>)> for ProgramParser {
    fn parse(&self, pair: ParsePair<'_>, _ctx: &mut ParseContext) -> ParseResult<(Program, BTreeMap<usize, String>)> {
        let mut ctx = ParseContext::new();
        let mut functions = BTreeMap::new();
        let mut function_locations = BTreeMap::new();

        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::constant_declaration => {
                    let (name, value) = ConstantDeclarationParser.parse(item, &mut ctx)?;
                    match value {
                        ParsedConstant::Scalar(v) => ctx.add_constant(name, v)?,
                        ParsedConstant::Array(arr) => ctx.add_const_array(name, arr)?,
                    }
                }
                Rule::import_statement => {
                    // Visit the imported file and parse it into the context
                    // and program; also keep track of which files have been
                    // imported and do not import the same file twice.
                    todo!()
                }
                Rule::function => {
                    let location = item.line_col().0;
                    let function = FunctionParser.parse(item, &mut ctx)?;
                    let name = function.name.clone();

                    function_locations.insert(location, name.clone());

                    if functions.insert(name.clone(), function).is_some() {
                        return Err(SemanticError::with_context(
                            format!("Multiply defined function: {name}"),
                            "function definition",
                        )
                        .into());
                    }
                }
                Rule::EOI => break,
                _ => {} // Skip other rules
            }
        }

        Ok((
            Program {
                functions,
                const_arrays: ctx.const_arrays,
            },
            function_locations,
        ))
    }
}

/// Parser for import statements.
pub struct ImportStatementParser;

impl Parse<String> for ImportStatementParser {
    fn parse(pair: ParsePair<'_>, ctx: &mut ParseContext) -> ParseResult<String> {
        let mut inner = pair.into_inner();
        let mut item = next_inner_pair(&mut inner, "filepath")?;
        match item.as_rule() {
            Rule::filepath => {
                let mut inner = item.into_inner();
                let mut filepath = String::new();
                for item in inner {
                    match item.as_rule() {
                        Rule::filepath_character => {
                            filepath.push_str(item.as_str());
                        },
                        _ => {
                            return Err(SemanticError::with_context(
                                format!("Expected a filepath character, got: {}",
                                    item.as_str()),
                                "filepath character",
                            ).into());
                        }
                    }
                }
                Ok(filepath)
            },
            _ => Err(SemanticError::with_context(
                format!("Expected a filepath, got: {}", item.as_str()),
                "filepath"
            )
            .into()),
        }
    }
}


