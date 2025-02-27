use asm_virtual_machine::{machine::Machine, parser::parse_program};
use log::info;
use register_list::RegisterList;
use textfield::TextField;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

use yew::prelude::*;

pub mod textfield;

pub mod register_list;
pub mod register_selector;

pub struct App {
    pub log: Vec<String>,
    pub machine: Machine,
    pub program: String,
    pub text_ref: NodeRef,
    pub check_ref: NodeRef,
}

pub enum Msg {
    RunProgram,
    ClearLog,
    SetRegister(usize, i8),
    GetRegister,
    ResetRegisters,
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let log = Vec::new();
        let machine = Machine::new();
        let program = String::default();
        let text_ref = NodeRef::default();
        let check_ref = NodeRef::default();
        App {
            log,
            machine,
            program,
            text_ref,
            check_ref,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RunProgram => {
                let input_val = &self.text_ref.cast::<HtmlTextAreaElement>().unwrap().value();
                let verbose = &self.check_ref.cast::<HtmlInputElement>().unwrap().checked();

                info!("{}", input_val.to_string());

                let Ok(program) = parse_program(input_val) else {
                    return true;
                };
                self.machine.init_program(program);
                let mut r = Ok(());
                while r == Ok(()) {
                    let ins = self.machine.get_current_instruction();
                    r = self.machine.step();
                    let regs = self.machine.get_string_registers();
                    if *verbose {
                        self.log.push(ins);
                        self.log.push(regs);
                    }
                }
                if !*verbose {
                    let regs = self.machine.get_string_registers();
                    self.log.push(regs);
                }

                true
            }
            Msg::ClearLog => {
                self.log = Vec::new();
                true
            }
            Msg::SetRegister(r, v) => {
                let _ = self.machine.set_register(r, v);
                info!("Register R{} is now {}", r, v);
                false
            }
            Msg::GetRegister => {
                let reg_val = self.machine.get_string_registers();
                self.log.push(reg_val);
                true
            }
            Msg::ResetRegisters => {
                for i in 0..7 {
                    let _ = self.machine.set_register(i as usize, 0);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let log = self
            .log
            .iter()
            .map(|entry| {
                let entry = entry.split("\n").map(|line| html! {<p>{line}</p>});
                html! {
                    <div class={classes!("entry")}>
                    {for entry}
                    </div>
                }
            })
            .collect::<Html>();
        html! {
            <>
                <h1>{"Comparc Interpreter"}</h1>
                <TextField node_ref={&self.text_ref} />
                <div class={classes!("control")}>
                    <button onclick={ctx.link().callback(|_| Msg::RunProgram)}>{"Run Program"}</button>
                    <button onclick={ctx.link().callback(|_| Msg::ClearLog)}>{"Clear Log"}</button>
                    <button onclick={ctx.link().callback(|_| Msg::GetRegister)}>{"Print Register"}</button>
                    <button onclick={ctx.link().callback(|_| Msg::ResetRegisters)}>{"Reset Register"}</button>
                    <label for="verbose">{"Verbose mode"}</label>
                    <input type="checkbox" ref={&self.check_ref} name="verbose" />
                </div>
                <RegisterList callback={ctx.link().callback(|(r,v)| Msg::SetRegister(r,v))}/>
                <div class={classes!("log")}>
                    {log}
                </div>
            </>
        }
    }
}
