#![allow(non_camel_case_types)]

use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ArrowDownIconProps {
    #[prop_or_default]
    class: &'static str,
}

#[function_component(ArrowDownIcon)]
fn arrow_down_icon(props: &ArrowDownIconProps) -> Html {
    html! {
        <svg aria-hidden="true" viewBox="0 0 24 24" class={props.class}>
            <path
                d="m17 14-5 5-5-5M12 18.5V5"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
        </svg>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ExpandableProps {
    pub is_expanded: UseStateHandle<bool>,
    pub expand: Callback<MouseEvent>,
    pub children: Children,
}

#[function_component(Expandable)]
pub fn expandable(props: &ExpandableProps) -> Html {
    html! {
        <>
            { for props.children.iter() }
            if !*props.is_expanded {
                <ExpandableButton expand={props.expand.clone()} />
            }
        </>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ExpandableButtonProps {
    pub expand: Callback<MouseEvent>,
}

#[function_component(ExpandableButton)]
pub fn expandable_button(props: &ExpandableButtonProps) -> Html {
    html! {
        <div class="mt-10 flex justify-center">
            <button onclick={props.expand.clone()} class="flex items-center text-base font-medium tracking-tight text-slate-900 hover:text-slate-700">
                { "Ko'proq ko'rsatish" }
                <ArrowDownIcon class="ml-2 h-6 w-6" />
            </button>
        </div>
    }
}
