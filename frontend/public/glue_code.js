


const invoke = window.__TAURI__.invoke

export function invoke_test_doctype(input) {
	return invoke("test_doctype", {input: input});
}


