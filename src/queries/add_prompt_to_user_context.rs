pub static ADD_PROMPT_TO_USER_CONTEXT_QUERY: &str =
    "UPDATE user_context SET chat = chat || ? WHERE user_id=?";
