use yew::{html, Component, Context, Html};
mod safehtml;
use safehtml::SafeHtml;

//has to fetch from an address that allows CORS
const ADDRESS: &str = "https://raw.githubusercontent.com/yewstack/yew/master/README.md";

struct FetchError;

async fn fetch_html( url: &str )  -> Result<String, FetchError>{

    let client = reqwest::Client::new();

    let future = client.get(url)
        .send();

    if let Ok(response) = future.await{
        return Ok( response.text().await.unwrap()  );
    }
    return Err( FetchError );
}

enum Msg {
    FetchHtml,
    SetHtml(String),
    Error,
}

struct App {
    fetchedhtml: Option<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {

        Self {
            fetchedhtml: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchHtml => {
                //runs the future in the background
                ctx.link().send_future(async {

                    if let Ok( html ) = fetch_html( ADDRESS ).await{
                        Msg::SetHtml( html )
                    }
                    else{
                        Msg::Error
                    }
                });
                false
            }
            Msg::SetHtml( html ) => {
                self.fetchedhtml = Some(html);
                true
            }
            Msg::Error => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(html) = &self.fetchedhtml{
            html!{
                <SafeHtml html={ html.clone() } />
            }
        }
        else{

            html!{
                <button onclick={ctx.link().callback(|_| Msg::FetchHtml)}>
                    { "Fetch Html" }
                </button>
            }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}