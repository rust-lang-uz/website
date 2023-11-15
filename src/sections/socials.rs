use crate::functions::*;
use yew::prelude::*;

#[function_component(Socials)]
pub fn socials() -> Html {
    html! {
        <section
            id="socials"
            aria-label="Social Networks"
            class="scroll-mt-14 bg-orange-600 sm:scroll-mt-32"
        >
            <div class="overflow-hidden lg:relative">
                <Container size="md" class="relative grid grid-cols-1 items-end gap-y-12 py-20 lg:static lg:grid-cols-2 lg:py-28 xl:py-32">
                    <Pattern class="absolute -top-32 left-0 w-full sm:-top-5 sm:left-3/4 sm:ml-8 sm:w-auto md:left-2/3 lg:left-auto lg:right-2 lg:ml-0 xl:left-2/3 xl:right-auto" />
                    <div>
                        <h2 class="font-display text-5xl font-extrabold tracking-tight text-white sm:w-3/4 sm:text-6xl md:w-2/3 lg:w-auto">
                            { "Bizni kuzatib boring!" }
                        </h2>
                        <p class="mt-4 text-lg tracking-tight text-orange-200">
                            { "Rust o'z hamjamiyati haqida qayg'uradi va o'z ijtimoiy tarmoqlarida barcha aktual yangiliklar bilan ulashib boradi." }
                        </p>
                    </div>
                    <div class="lg:pl-16">
                        <h3 class="text-base font-medium tracking-tight text-white">
                            { "Twitterdagi tarmoqlarimiz " }<span aria-hidden="true">{ "→" }</span>
                        </h3>
                        <div class="mt-4 sm:relative sm:flex sm:items-center sm:py-0.5 sm:pr-2.5">
                            // Button components with their respective props
                            <Button href="https://twitter.com/rustlang" color="white" variant="solid" class="mx-2 mt-4 w-full sm:relative sm:z-10 sm:mt-0 sm:w-auto sm:flex-none">
                                { "@rustlang" }
                            </Button>
                            <Button href="https://twitter.com/rust_foundation" color="white" variant="solid" class="mx-2 mt-4 w-full sm:relative sm:z-10 sm:mt-0 sm:w-auto sm:flex-none">
                                { "@rust_foundation" }
                            </Button>
                            <Button href="https://twitter.com/cratesiostatus" color="white" variant="solid" class="mx-2 mt-4 w-full sm:relative sm:z-10 sm:mt-0 sm:w-auto sm:flex-none">
                                { "@cratesiostatus" }
                            </Button>
                        </div>
                    </div>
                </Container>
            </div>
        </section>
    }
}
