use stuart_core::functions::{Function, FunctionParser};
use stuart_core::parse::{ParseError, RawFunction};
use stuart_core::process::{ProcessError, Scope};
use stuart_core::{declare_plugin, quiet_assert, TracebackError};

use mathjax_svg::{convert_to_svg, convert_to_svg_inline};

declare_plugin! {
    name: "stuart_math",
    version: "0.1.0",
    functions: [DisplayMathParser, InlineMathParser],
    parsers: [],
}

pub struct DisplayMathParser;
pub struct InlineMathParser;

#[derive(Debug, Clone)]
pub struct DisplayMathFunction {
    math: String,
}

#[derive(Debug, Clone)]
pub struct InlineMathFunction {
    math: String,
}

impl FunctionParser for DisplayMathParser {
    fn name(&self) -> &'static str {
        "math"
    }

    fn parse(&self, raw: RawFunction) -> Result<Box<dyn Function>, ParseError> {
        quiet_assert!(raw.positional_args.len() == 1)?;

        let math = raw.positional_args[0]
            .as_string()
            .ok_or(ParseError::InvalidArgument)?
            .to_string();

        Ok(Box::new(DisplayMathFunction { math }))
    }
}

impl FunctionParser for InlineMathParser {
    fn name(&self) -> &'static str {
        "mathi"
    }

    fn parse(&self, raw: RawFunction) -> Result<Box<dyn Function>, ParseError> {
        quiet_assert!(raw.positional_args.len() == 1)?;

        let math = raw.positional_args[0]
            .as_string()
            .ok_or(ParseError::InvalidArgument)?
            .to_string();

        Ok(Box::new(InlineMathFunction { math }))
    }
}

impl Function for DisplayMathFunction {
    fn name(&self) -> &'static str {
        "math"
    }

    fn execute(&self, scope: &mut Scope) -> Result<(), TracebackError<ProcessError>> {
        let self_token = scope.tokens.current().unwrap().clone();
        let svg = convert_to_svg(&self.math)
            .map_err(|_| self_token.traceback(ProcessError::StackError))?;

        scope.output(svg).map_err(|e| self_token.traceback(e))
    }
}

impl Function for InlineMathFunction {
    fn name(&self) -> &'static str {
        "mathi"
    }

    fn execute(&self, scope: &mut Scope) -> Result<(), TracebackError<ProcessError>> {
        let self_token = scope.tokens.current().unwrap().clone();
        let svg = convert_to_svg_inline(&self.math)
            .map_err(|_| self_token.traceback(ProcessError::StackError))?;

        scope.output(svg).map_err(|e| self_token.traceback(e))
    }
}
