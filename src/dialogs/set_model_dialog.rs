pub fn get_dialog(server_name: String) -> String {
    let message = format!(
        "The model has been configured correctly for the server {} ✅",
        server_name
    );

    message
}
