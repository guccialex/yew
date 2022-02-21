use yew::{html, Component, Context, Html};


//fetch from the trunk server, the data in the files
async fn fetch_yew_readme( )  -> Option<String>{

    //whatever api you call must have the same origin or allow CORS
    let url = "https://raw.githubusercontent.com/yewstack/yew/master/README.md";

    let client = reqwest::Client::new();

    let future = client.get(url)
        .send();

    if let Ok(response) = future.await{

        if let Ok(html) = response.text().await{

            return Some( html );
        }
    }

    return None;
}

enum Msg {
    FetchHtml,
    SetHtml( String ),
    Error,
}

struct App {

    htmltorender: Option<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {

        Self {
            htmltorender: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchHtml => {

                //runs the future in the background and when the async function returns
                //it calls update on this component with the returned message
                ctx.link().send_future(async {

                    if let Some( html ) = fetch_yew_readme( ).await{
                        Msg::SetHtml( html )
                    }
                    else{
                        Msg::Error
                    }
                });
                false
            }
            Msg::SetHtml( html ) => {
                self.htmltorender = Some(html);
                true
            }
            Msg::Error => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        if let Some(html) = &self.htmltorender{

            //Set the html of the body of the document            
            gloo_utils::body().set_inner_html(&html.clone());

            html!{}
        }
        else{

            html!{
                <>
                    <button onclick={ctx.link().callback(|_| Msg::FetchHtml)}>
                        { "Fetch and render the html in the Yew readme" }
                    </button>
                </>
            }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}