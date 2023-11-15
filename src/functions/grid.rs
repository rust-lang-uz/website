use web_sys::window;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct GridPatternProps {
    #[prop_or_default]
    pub x: &'static str,
    #[prop_or("100")]
    pub y: &'static str,
    #[prop_or_default]
    pub pattern_transform: &'static str,
}

#[function_component(GridPattern)]
pub fn grid_pattern(props: &GridPatternProps) -> Html {
    let pattern_id = {
        let window = window().expect("no global `window` exists");
        let crypto = window.crypto().expect("should have `crypto` on `window`");
        crypto.random_uuid()
    };

    html! {
        <svg aria-hidden="true" class="absolute inset-0 h-full w-full">
            <defs>
                <pattern
                    id={pattern_id.clone()}
                    width="128"
                    height="128"
                    patternUnits="userSpaceOnUse"
                    x={props.x}
                    y={props.y}
                    patternTransform={props.pattern_transform}
                >
                    <path d="M0 128V.5H128" fill="none" stroke="currentColor" />
                </pattern>
            </defs>
            <rect width="100%" height="100%" fill={format!("url(#{})", pattern_id)} />
        </svg>
    }
}
