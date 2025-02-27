use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub register: usize,
    pub callback: Callback<i8>,
}

pub struct RegisterSelector {
    pub node_ref: NodeRef,
}

impl Component for RegisterSelector {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        let Some(val) = &self
            .node_ref
            .cast::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<i8>()
            .ok()
        else {
            return false;
        };

        ctx.props().callback.emit(*val);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reg = ctx.props().register;
        html! {
            <div class={classes!("register")}>
                <label for={format!("R{}", reg)}>{ format!("R{} =", reg) }</label>
                <input
                type="number"
                name={format!("R{}", reg)}
                oninput={ctx.link().callback(|_| ())}
                ref={&self.node_ref}
                />
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.node_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .set_value("0");
        }
    }
}
