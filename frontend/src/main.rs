


use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use yew::prelude::*;

mod text_input;
use text_input::TextInput;



fn main() {
	yew::start_app::<App>();
}



#[wasm_bindgen(module = "/public/glue_code.js")]
extern "C" {
	#[wasm_bindgen(js_name = invoke_test_doctype, catch)]
	pub fn test_doctype(input: String) -> Result<JsValue, JsValue>;
}

async fn get_output(input: String) -> String {
	match test_doctype(input) {
		Ok(val) => {
			let promise = Promise::resolve(&val);
			web_sys::console::log_1(&promise);
			match JsFuture::from(promise).await {
				Ok(s) => s.as_string().unwrap(),
				Err(e) => e.as_string().unwrap()
			}
		},
		Err(e) => {
			let window = window().unwrap();
			window
				.alert_with_message(&format!("Error: {:?}", e))
				.unwrap();
			e.as_string().unwrap()
		}
	}
}



pub enum Msg {
	UpdateText(String),
	SetOutput(String)
}

pub struct App {
	text: String,
	output: String
}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create (_ctx: &Context<Self>) -> Self {
		let text = String::new();
		let output = String::new();
		Self { text, output }
	}


	fn update (&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::UpdateText(new_text) => {
				self.text = new_text;
				let text = self.text.clone();
				ctx.link().send_future(async {
					Msg::SetOutput(get_output(text).await)
				});
				false
			},
			Msg::SetOutput(output) => {
				self.output = output;
				true
			}
		}
	}


	fn view (&self, ctx: &Context<Self>) -> Html {

		let on_change = ctx.link().callback(Msg::UpdateText);

		html! {
			<main>
				<div id="container">
					<TextInput {on_change} value={self.text.clone()}/>
					<pre>{ self.output.clone() }</pre>
				</div>
			</main>
		}

	}

}


