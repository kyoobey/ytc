


use wasm_bindgen::{ JsCast, UnwrapThrowExt };
use web_sys::{ Event, HtmlTextAreaElement, InputEvent };
use yew::prelude::*;



#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	pub value: String,
	pub on_change: Callback<String>
}



fn get_value_from_input_event (e: InputEvent) -> String {
	// web_sys::console::log_1(&JsValue::from_str("throw 0"));
	let event: Event = e.dyn_into().unwrap_throw();
	let event_target = event.target().unwrap_throw();
	let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
	target.value()
}



#[function_component(TextInput)]
pub fn text_input (props: &Props) -> Html {
	let Props { value, on_change } = props.clone();

	let oninput = Callback::from(move |input_event: InputEvent| {
		on_change.emit(get_value_from_input_event(input_event));
	});

	html! {
		<textarea type="text" {value} {oninput}/>
	}
}


