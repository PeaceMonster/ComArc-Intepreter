use yew::prelude::*;

pub struct TextField;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub node_ref: NodeRef,
}

impl Component for TextField {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <textarea class={classes!("program")}
                ref={&ctx.props().node_ref}
                type="text"
                />
        }
    }
}
