// Messages should be Cloned because they represent pure data/events
// button needs it too, because there is a chance that you will press it multiple times
// between calls

// Defines the types of messages or events that
// the calculator will handle
#[derive(Debug, Clone)]
pub enum CalculatorMessage {
    Calculate
}