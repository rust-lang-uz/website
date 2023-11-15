use crate::sections::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Hero />
            <Introduction />
            <NavBar />
            <Projects />
            <Resources />
            <Communities />
            <Socials />
            <Contributor />
            <Footer />
        </main>
    }
}
