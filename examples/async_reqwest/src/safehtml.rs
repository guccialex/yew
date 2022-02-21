//original author of this code is https://github.com/noxue
//at https://github.com/yewstack/yew/issues/1281

use yew::{function_component, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SafeProps {
    pub html: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &SafeProps) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());

    Html::VRef(div.into())
}