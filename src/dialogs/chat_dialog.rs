pub fn get_dialog(response: String, username: String) -> String {
    let message = format!("Saving in the {} context 💫 \n {}", username, response);
    message
}
