use stuart_core::functions::{Function, FunctionParser};
use stuart_core::parse::{ParseError, RawFunction};
use stuart_core::process::{ProcessError, Scope};
use stuart_core::{declare_plugin, quiet_assert, TracebackError};

use mathjax_svg::{convert_to_svg, convert_to_svg_inline};
use xmltree::Element;

declare_plugin! {
    name: "stuart_math",
    version: env!("CARGO_PKG_VERSION"),
    functions: [DisplayMathParser, InlineMathParser],
    parsers: [],
}

static SVG_STYLE: &str = "display:block;margin:auto;";

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
        let mut svg = convert_to_svg(&self.math)
            .map_err(|_| self_token.traceback(ProcessError::StackError))?;

        // Make display math block and centered.
        let mut element = Element::parse(svg.as_bytes())
            .map_err(|_| self_token.traceback(ProcessError::StackError))?;
        if let Some(style) = element.attributes.get_mut("style") {
            *style = SVG_STYLE.to_string() + style;
        } else {
            element
                .attributes
                .insert("style".to_string(), SVG_STYLE.to_string());
        }

        let mut buf = Vec::new();
        element
            .write(&mut buf)
            .map_err(|_| self_token.traceback(ProcessError::StackError))?;
        svg = String::from_utf8(buf).unwrap();

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
