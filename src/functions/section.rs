use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionHeadingProps {
    #[prop_or_default]
    pub number: String,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(SectionHeading)]
pub fn section_heading(props: &SectionHeadingProps) -> Html {
    let builtin = "inline-flex items-center rounded-full px-4 py-1 text-blue-600 ring-1 ring-inset ring-blue-600";
    let class_name = classes!(&props.class, builtin,);

    html! {
        <h2 class={class_name} id={props.id.clone()}>
            <span class="font-mono text-sm" aria-hidden="true">
                { format!("{:0>2}", props.number) }
            </span>
            <span class="ml-3 h-3.5 w-px bg-blue-600/20" />
            <span class="ml-3 text-base font-medium tracking-tight">
                { for props.children.iter() }
            </span>
        </h2>
    }
}
