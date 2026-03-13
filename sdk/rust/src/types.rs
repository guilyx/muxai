use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ProviderName {
    Cursor,
    Claude,
    Vibe,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FinishReason {
    Stop,
    ToolCall,
    Length,
    Error,
    Incomplete,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
}

impl Default for Usage {
    fn default() -> Self {
        Self {
            input_tokens: 0,
            output_tokens: 0,
            total_tokens: 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    pub messages: Vec<Message>,
    pub system_prompt: Option<String>,
    pub tools: Vec<ToolDefinition>,
    pub max_turns: u32,
    pub metadata: HashMap<String, String>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            system_prompt: None,
            tools: Vec::new(),
            max_turns: 1,
            metadata: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    pub provider: ProviderName,
    pub content: String,
    pub raw: String,
    pub finish_reason: FinishReason,
    pub usage: Usage,
    pub tool_calls: Vec<ToolCall>,
    pub duration: Duration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventType {
    Started,
    Delta,
    Done,
    Error,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    pub event_type: EventType,
    pub provider: ProviderName,
    pub delta: Option<String>,
    pub response: Option<Response>,
    pub error: Option<String>,
}
