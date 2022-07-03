


#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]



fn main() {
	let context = tauri::generate_context!();
	tauri::Builder::default()
		.menu(if cfg!(target_os = "macos") {
			tauri::Menu::os_default(&context.package_info().name)
		} else {
			tauri::Menu::default()
		})
		.invoke_handler(tauri::generate_handler![test_doctype])
		.run(context)
		.expect("error while running tauri application");
}



#[tauri::command]
fn test_doctype(input: String) -> Result<String, String> {
	if input.contains("doctype") {
		Ok(input.to_string())
	} else {
		Err("doctype ?".to_string())
	}
}


