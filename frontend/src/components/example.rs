use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;

use common::Example;

pub struct ExampleComponent {
    example: Option<Example>,
    input: String,
}

pub enum ExampleMsg {
    Input(String),
    Request,
    Response(Example),
}

impl Component for ExampleComponent {
    type Message = ExampleMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { example: None, input: String::new() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ExampleMsg::Input(input) => {
                self.input = input;
                false
            }
            ExampleMsg::Request => {
                let input = self.input.clone();
                let scope = ctx.link().clone();

                spawn_local(async move {
                    let response = reqwest::Client::new()
                        .get(format!("http://127.0.0.1:8080/api/v1/hello/{}", input))
                        .send()
                        .await
                        .unwrap()
                        .json::<Example>()
                        .await
                        .unwrap();

                    scope.send_message(ExampleMsg::Response(response));
                });
                false
            }
            ExampleMsg::Response(example) => {
                self.example = Some(example);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(move |event: InputEvent| {
            let value = event.target_dyn_into::<HtmlInputElement>().unwrap().value();
            ExampleMsg::Input(value)
        });

        let onsubmit = ctx.link().callback(move |event: SubmitEvent| {
            event.prevent_default();
            ExampleMsg::Request
        });

        html! {
                <div>
                    <p>{match &self.example {
                        None => "".to_string(),
                        Some(value) => value.string.clone()
                    }}</p>
                    <form {onsubmit}>
                        <input type="text" {oninput} />
                        <input type="submit" />
                    </form>
                </div>
            }
    }
}
