/// Contains data that identifies the command, query or request that's being executed.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Context {
    pub request_id: String,
}
