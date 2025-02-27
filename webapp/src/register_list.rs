use yew::prelude::*;

use crate::register_selector::RegisterSelector;

pub struct RegisterList;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub callback: Callback<(usize, i8)>,
}

impl Component for RegisterList {
    type Message = (usize, i8);

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        ctx.props().callback.emit(msg);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("regs")}>
                <RegisterSelector register={0} callback={ctx.link().callback(|v| (0,v))} />
                <RegisterSelector register={1} callback={ctx.link().callback(|v| (1,v))} />
                <RegisterSelector register={2} callback={ctx.link().callback(|v| (2,v))} />
                <RegisterSelector register={3} callback={ctx.link().callback(|v| (3,v))} />
                <RegisterSelector register={4} callback={ctx.link().callback(|v| (4,v))} />
                <RegisterSelector register={5} callback={ctx.link().callback(|v| (5,v))} />
                <RegisterSelector register={6} callback={ctx.link().callback(|v| (6,v))} />
                <RegisterSelector register={7} callback={ctx.link().callback(|v| (7,v))} />
            </div>
        }
    }
}
