#![allow(suspicious_double_ref_op)]

use crate::functions::*;
use yew::prelude::*;

// Static vector of ...
type Obscure = &'static [(&'static str, &'static [(&'static str, &'static str)])];

const DATA: Obscure = &[
    (
        "Buyruq satri",
        &[
            ("Sodda va tez", "https://www.rust-lang.org/learn"),
            (
                "Oson ishga tushuvchan",
                "https://rust-cli.github.io/book/tutorial/packaging.html",
            ),
            (
                "Tushunarli konfiguratsiya",
                "https://rust-cli.github.io/book/in-depth/config-files.html",
            ),
            (
                "Qo'llanma? Tayyor",
                "https://rust-cli.github.io/book/in-depth/docs.html",
            ),
            (
                "Oson logginglar",
                "https://rust-cli.github.io/book/in-depth/human-communication.html",
            ),
        ],
    ),
    (
        "Webassembly",
        &[
            ("Bashoratli tezlik", "https://webassembly.org"),
            (
                "Kichik dastur hajmi",
                "https://rustwasm.github.io/docs/book",
            ),
            (
                "Zamonaviy qulayliklar",
                "https://developer.mozilla.org/en-US/docs/WebAssembly",
            ),
        ],
    ),
    (
        "Tarmog'lar",
        &[
            ("Kam iz", "https://docs.rs/reqwest"),
            ("Xavfsiz va ishonchli", "https://rocket.rs/"),
            (
                "O'suvchan konkurrensiya",
                "https://www.rust-lang.org/what/networking",
            ),
        ],
    ),
    (
        "Embed",
        &[
            (
                "Kuchli statistik tahlil",
                "https://docs.rust-embedded.org/book/static-guarantees",
            ),
            (
                "Moslashuvchan xotira",
                "https://docs.rust-embedded.org/book/collections/",
            ),
            (
                "Qo'rquvsiz muvofiqlik",
                "https://docs.rust-embedded.org/book/concurrency",
            ),
        ],
    ),
];

#[function_component(Projects)]
pub fn projects() -> Html {
    let is_expanded: UseStateHandle<bool> = use_state(|| false);

    let expand = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_: MouseEvent| is_expanded.set(true))
    };

    html! {
        <section id="projects" aria-labelledby="projects-title" class="scroll-mt-14 py-16 sm:scroll-mt-32 sm:py-20 lg:py-32">
            <Container>
                <SectionHeading number="1" id="table-of-contents-title">
                    {"Loyihalar"}
                </SectionHeading>
                <p class="mt-8 font-display text-4xl font-bold tracking-tight text-slate-900">
                    {"Rust qamrab oluvchi yo'nalishlar"}
                </p>
                <p class="mt-4 text-lg tracking-tight text-slate-700">
                    {"Rust dasturlash tilida bir necha yo'nalishlar bo'yicha loyiha va dasturlar yozsa bo'ladi. Ushbu keltirilgan ro'yxatda ushbu yo'nalishlarni ko'rishingiz mumkin."}
                </p>
                <Expandable
                    is_expanded={is_expanded.clone()}
                    expand={expand}
                >
                    <ol role="list" class="mt-16 space-y-10 sm:space-y-16">
                        {
                            for DATA.iter().take(if *is_expanded { DATA.len() } else { 2 }).map(|(category, links)| {
                                html! {
                                    <li>
                                        <h3 class="font-display text-3xl font-bold tracking-tight text-slate-900">{category}</h3>
                                        <ol class="mt-8 divide-y divide-slate-300/30 rounded-2xl bg-slate-50 px-6 py-3 text-base tracking-tight sm:px-8 sm:py-7">
                                            { for links.iter().map(|(title, link)| {
                                                html! {
                                                    <li
                                                        class="flex justify-between py-3"
                                                        aria-label={format!("{} on page {}", title, link)}
                                                    >
                                                        <span
                                                            class="font-medium text-slate-900"
                                                            aria-hidden="true"
                                                        >
                                                            {title}
                                                        </span>

                                                        <span
                                                            class="font-mono text-slate-400"
                                                            aria-hidden="true"
                                                        >
                                                            <a href={link.clone()}>{"Ko'proq"}</a>
                                                        </span>

                                                    </li>
                                                }
                                            }) }
                                        </ol>
                                    </li>
                                }
                            })
                        }
                    </ol>
                </Expandable>
            </Container>
        </section>
    }
}
