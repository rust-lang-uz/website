use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlElement};
use yew::prelude::*;

type LessObscure = &'static [(&'static str, &'static str)];

const SECTIONS: LessObscure = &[
    ("projects", "Loyihalar"),
    ("resources", "Resurslar"),
    ("communities", "Jamiyatlar"),
    ("socials", "Sotsial tarmoqlar"),
    ("contributor", "Faol Rustacean"),
];

#[derive(Properties, PartialEq)]
struct MenuIconProps {
    #[prop_or_default]
    class: Classes,
    #[prop_or_default]
    open: bool,
}

#[function_component(MenuIcon)]
fn menu_icon(props: &MenuIconProps) -> Html {
    html! {
        <svg
            aria-hidden="true"
            fill="none"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            view-box="0 0 24 24"
            class={classes!("w-6", "h-6", props.class.clone())}
        >
            <path
                d={if props.open {
                    "M17 7 7 17M7 7l10 10"
                } else {
                    "m15 16-3 3-3-3M15 8l-3-3-3 3"
                }}
            />
        </svg>
    }
}

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let active_index: UseStateHandle<Option<usize>> = use_state(|| None::<usize>);
    let nav_bar_ref = use_node_ref();
    let is_popover_open = use_state(|| false);

    let toggle_popover = {
        let is_popover_open = is_popover_open.clone();
        Callback::from(move |_| is_popover_open.set(!*is_popover_open))
    };

    use_effect_with_deps(
        {
            let nav_bar_ref = nav_bar_ref.clone();
            let active_index = active_index.clone();

            move |_| {
                let handler = move || {
                    let window = window().expect("should have a window in this context");
                    let nav_bar_element = nav_bar_ref.cast::<HtmlElement>().unwrap();
                    let offset = nav_bar_element.offset_height() as f64;
                    let body_rect = window.document().unwrap().body().unwrap().get_bounding_client_rect();
                    let adjusted_offset = body_rect.top() + offset + 1.0;
                    let mut new_active_index: Option<usize> = None;
                
                    let scroll_y = window.scroll_y().unwrap(); // Convert JsValue to f64
                    let inner_height = window.inner_height().unwrap().as_f64().unwrap_or_default(); // Convert JsValue to f64
                
                    if scroll_y >= (body_rect.height() - inner_height).floor() {
                        new_active_index = Some(SECTIONS.len() - 1);
                    } else {
                        for (index, (id, _)) in SECTIONS.iter().enumerate() {
                            if let Some(element) = window.document().unwrap().get_element_by_id(id) {
                                let element = element.dyn_into::<Element>().unwrap();
                                let rect = element.get_bounding_client_rect();
                                if scroll_y >= rect.top() - adjusted_offset {
                                    new_active_index = Some(index);
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                                
                    active_index.set(new_active_index);
                };

                let handler = Closure::wrap(Box::new(handler) as Box<dyn Fn()>);

                if let Some(window) = window() {
                    window
                        .add_event_listener_with_callback(
                            "scroll",
                            handler.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    handler.forget();
                }

                || ()
            }
        },
        (),
    );

    html! {
        <div ref={nav_bar_ref} class="sticky top-0 z-50">
            // <Popover>
            <div class="sm:hidden">
                <div
                    class={classes!(
                        "relative flex items-center px-4 py-3".to_string(),
                        (!*is_popover_open).then_some("bg-white/95 shadow-sm [@supports(backdrop-filter:blur(0))]:bg-white/80 [@supports(backdrop-filter:blur(0))]:backdrop-blur"),
                    )}
                >
                    {
                        if !*is_popover_open {
                            let (section_number, section_title) = active_index
                                .map(|index| (index + 1, SECTIONS[index].1))
                                .unwrap_or((1, SECTIONS[0].1)); // Default to first section
            
                            html! {
                                <div>
                                    <span aria-hidden="true" class="font-mono text-sm text-blue-600">
                                        {format!("{:02}", section_number)}
                                    </span>
                                    <span class="ml-4 text-base font-medium text-slate-900">
                                        {section_title}
                                    </span>
                                </div>
                            }
                        } else {html! {}}
                    }
                    // <Popover Button>
                    <button 
                        onclick={toggle_popover.clone()} 
                        class={classes!(
                            "-mr-1 ml-auto flex h-8 w-8 items-center justify-center".to_string(),
                            ( if *is_popover_open { "relative z-10" } else { "" }).to_string()
                        )} 
                        aria-label="Toggle navigation menu"
                    >
                        {
                            if *is_popover_open {
                                html! {
                                    <div>
                                        <span class="absolute inset-0" />
                                    </div>
                                }
                            } else { html!{} }
                        }
                        <MenuIcon class="h-6 w-6 stroke-slate-700" open={*is_popover_open} />
                    </button>
                    // </Popover Button>
                </div>
                // <Popover Panel>
                if *is_popover_open {
                    <div class="absolute inset-x-0 top-0 bg-white/95 py-3.5 shadow-sm [@supports(backdrop-filter:blur(0))]:bg-white/80 [@supports(backdrop-filter:blur(0))]:backdrop-blur">
                        {
                            for SECTIONS.iter().enumerate().map(|(index, (id, title))| {
                                let active_index = active_index.clone();
                                let toggle_popover = toggle_popover.clone();

                                let on_nav_click = {
                                    Callback::from(move |_| {
                                        active_index.set(Some(index));
                                        toggle_popover.emit(MouseEvent::new("click").unwrap());
                                    })
                                };
                                                        
                                html! {
                                    <a 
                                        href={format!("#{}", id)}
                                        class="flex items-center px-4 py-1.5" 
                                        onclick={on_nav_click}
                                    >
                                        <span
                                            aria-hidden="true"
                                            class="font-mono text-sm text-blue-600"
                                        >
                                            {format!("{:02}", SECTIONS.iter().position(|(section_id, _)| section_id == id).unwrap() + 1)}
                                        </span>
                                        <span
                                            class="ml-4 text-base font-medium text-slate-900"
                                        >
                                            {title}
                                        </span>
                                    </a>
                                }
                            })
                        }
                    </div>
                }
                // </Popover Panel>
                <div class="absolute inset-x-0 bottom-full z-10 h-4 bg-white" />
            </div>
            // </Popover>
            <div class="hidden sm:flex sm:h-32 sm:justify-center sm:border-b sm:border-slate-200 sm:bg-white/95 sm:[@supports(backdrop-filter:blur(0))]:bg-white/80 sm:[@supports(backdrop-filter:blur(0))]:backdrop-blur">
                <ol
                    role="list"
                    class="mb-[-2px] grid auto-cols-[minmax(0,15rem)] grid-flow-col text-base font-medium text-slate-900 [counter-reset:section]"
                >
                    {
                        for SECTIONS.iter().enumerate().map(|(index, (id, title))| {
                            html! {
                                <li
                                    key={id.to_string()}
                                    class="flex [counter-increment:section]"
                                >
                                    <a
                                        href={format!("#{}", id)}
                                        class={
                                            classes!("flex w-full flex-col items-center justify-center border-b-2 before:mb-2 before:font-mono before:text-sm before:content-[counter(section,decimal-leading-zero)]".to_string(),
                                                (if *active_index == Some(index) {
                                                    "border-blue-600 bg-blue-50 text-blue-600 before:text-blue-600"
                                                } else {
                                                    "border-transparent before:text-slate-500 hover:bg-blue-50/40 hover:before:text-slate-900"
                                                }).to_string()
                                            )
                                        }
                                    >
                                        {title}
                                    </a>
                                </li>
                            }
                        })
                    }
                </ol>
            </div>
        </div>
    }
}
