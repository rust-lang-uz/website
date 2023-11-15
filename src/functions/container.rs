use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub size: Option<&'static str>,
    #[prop_or_default]
    pub class: Option<&'static str>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let size_class = match props.size {
        Some("xs") => "mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4 lg:px-2",
        Some("sm") => "mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4 lg:max-w-4xl lg:px-12",
        Some("md") => "mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4 lg:max-w-5xl lg:px-8",
        Some("lg") => "mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4 lg:max-w-7xl lg:px-8",
        _ => "mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4 lg:max-w-4xl lg:px-12",
    };

    let class_name = classes!(size_class, props.class);

    html! {
        <div class={class_name}>
            { props.children.clone() }
        </div>
    }
}
