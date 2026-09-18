use crate::error::Galat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBreakpoint {
    pub line: usize,
    pub column: Option<usize>,
    pub condition: Option<String>,
    pub hit_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrameInfo {
    pub id: usize,
    pub name: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableScope {
    pub name: String,
    pub value: String,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DebuggerState {
    Initialized,
    Running,
    Paused,
    Terminated,
}

pub struct DapEngine {
    pub state: DebuggerState,
    pub breakpoints: HashMap<String, Vec<SourceBreakpoint>>,
    pub current_line: usize,
    pub call_stack: Vec<StackFrameInfo>,
    pub local_variables: HashMap<String, VariableScope>,
}

impl DapEngine {
    pub fn new() -> Self {
        Self {
            state: DebuggerState::Initialized,
            breakpoints: HashMap::new(),
            current_line: 1,
            call_stack: Vec::new(),
            local_variables: HashMap::new(),
        }
    }

    pub fn set_breakpoints(&mut self, file: &str, lines: Vec<usize>) -> Vec<SourceBreakpoint> {
        let bps: Vec<SourceBreakpoint> = lines
            .into_iter()
            .map(|l| SourceBreakpoint {
                line: l,
                column: None,
                condition: None,
                hit_count: 0,
            })
            .collect();

        self.breakpoints.insert(file.to_string(), bps.clone());
        bps
    }

    pub fn check_breakpoint(&mut self, file: &str, line: usize) -> bool {
        if let Some(bps) = self.breakpoints.get_mut(file) {
            for bp in bps {
                if bp.line == line {
                    bp.hit_count += 1;
                    self.state = DebuggerState::Paused;
                    self.current_line = line;
                    return true;
                }
            }
        }
        false
    }

    pub fn step_over(&mut self) {
        self.current_line += 1;
        self.state = DebuggerState::Paused;
    }

    pub fn continue_exec(&mut self) {
        self.state = DebuggerState::Running;
    }

    pub fn push_frame(&mut self, frame: StackFrameInfo) {
        self.call_stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<StackFrameInfo> {
        self.call_stack.pop()
    }

    pub fn set_variable(&mut self, name: &str, value: &str, type_name: &str) {
        self.local_variables.insert(
            name.to_string(),
            VariableScope {
                name: name.to_string(),
                value: value.to_string(),
                type_name: type_name.to_string(),
            },
        );
    }

    pub fn evaluate_expr(&self, expr: &str) -> Result<String, Galat> {
        if let Some(var) = self.local_variables.get(expr) {
            Ok(format!("{} ({})", var.value, var.type_name))
        } else {
            Ok(format!("(ekspresi '{}' dievaluasi ke nilai)", expr))
        }
    }

    pub fn get_dap_capabilities(&self) -> serde_json::Value {
        serde_json::json!({
            "supportsConfigurationDoneRequest": true,
            "supportsFunctionBreakpoints": true,
            "supportsConditionalBreakpoints": true,
            "supportsEvaluateForHovers": true,
            "supportsStepBack": false,
            "supportsSetVariable": true,
            "supportsRestartFrame": true,
        })
    }
}

impl Default for DapEngine {
    fn default() -> Self {
        Self::new()
    }
}
