use crate::functions::GridPattern;
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let current_year = {
        let date = js_sys::Date::new_0();
        date.get_full_year()
    };

    html! {
        <footer class="relative pb-20 pt-5 sm:pb-32 sm:pt-14">
            <div class="absolute inset-x-0 top-0 h-32 text-slate-900/10 [mask-image:linear-gradient(white,transparent)]">
                <GridPattern x="50%" />
            </div>
            <div class="relative text-center text-sm text-slate-600">
                <p>
                    {"CC0-1.0 Litsenziya ostida © "}
                    {if current_year != 2023 { "2023 - " } else { "" }}
                    {current_year}
                    {" Rust O'zbek Hamjamiyati"}
                </p>
            </div>
        </footer>
    }
}
