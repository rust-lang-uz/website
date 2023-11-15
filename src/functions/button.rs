use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: String,
    #[prop_or_default]
    pub color: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let base_styles = match props.variant.as_str() {
        "outline" => "inline-flex justify-center rounded-md border py-[calc(theme(spacing.1)-1px)] px-[calc(theme(spacing.4)-1px)] text-base font-semibold tracking-tight focus:outline-none",
        _ => "inline-flex justify-center rounded-md py-1 px-4 text-base font-semibold tracking-tight shadow-sm focus:outline-none",
    };

    let variant_styles = match (props.variant.as_str(), props.color.as_str()) {
        ("outline", "blue") => "border-blue-300 text-blue-600 hover:border-blue-400 hover:bg-blue-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 active:text-blue-600/70 disabled:opacity-40 disabled:hover:border-blue-300 disabled:hover:bg-transparent",
        ("outline", "slate") => "border-slate-200 text-slate-900 hover:border-slate-300 hover:bg-slate-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-slate-600 active:border-slate-200 active:bg-slate-50 active:text-slate-900/70 disabled:opacity-40 disabled:hover:border-slate-200 disabled:hover:bg-transparent",
        ("solid", "blue") => "bg-blue-600 text-white hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 active:bg-blue-700 active:text-white/80 disabled:opacity-30 disabled:hover:bg-blue-600",
        ("solid", "white") => "bg-white text-blue-600 hover:text-blue-700 focus-visible:text-blue-900 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-white active:bg-blue-50 active:text-blue-900/80 disabled:opacity-40 disabled:hover:text-blue-600",
        _ => "bg-slate-900 text-white hover:bg-slate-700 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-slate-900 active:bg-slate-700 active:text-white/80 disabled:opacity-30 disabled:hover:bg-slate-900",
    };

    let class_name = classes!(base_styles, variant_styles, props.class.clone());

    html! {
        if let Some(href) = &props.href {
            <a href={href.clone()} class={class_name} color={props.color.clone()}>
                { props.children.clone() }
            </a>
        } else {
            <button class={class_name} color={props.color.clone()}>
                { props.children.clone() }
            </button>
        }
    }
}
