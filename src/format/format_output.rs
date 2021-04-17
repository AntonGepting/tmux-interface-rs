use crate::format::format::Format;
use crate::format::variable_output::VariableOutput;

#[derive(Debug)]
pub struct FormatOutput(Vec<VariableOutput>);

impl FormatOutput {
    // TODO: check vec same size, return type?
    // XXX: mb from_string for default format too?
    pub fn from_string_ext(s: &str, format: Format) -> Self {
        let mut output = Vec::new();
        let v: Vec<&str> = s.split(format.separator).collect();
        for (i, variable) in v.iter().enumerate() {
            let var = VariableOutput::from_string_ext(variable, &format.variables[i]);
            output.push(var);
        }
        Self(output)
    }
}
