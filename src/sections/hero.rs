use crate::functions::*;
use yew::prelude::*;

#[function_component(Testimonial)]
pub fn testimonial() -> Html {
    html! {
        <figure class="relative mx-auto max-w-md text-center lg:mx-0 lg:text-left">
            <div class="flex justify-center text-blue-600 lg:justify-start">
                <StarRating />
            </div>
            <blockquote class="mt-2">
                <p class="font-display text-xl font-medium text-slate-900">
                    { "“Rustda biron dasturni muvaffaqiyatli yozish uchun deyarli hamma kerakli dastur va asboblar mavjud”" }
                </p>
            </blockquote>
            <a href="https://orzklv.uz">
                <figcaption class="mt-2 text-sm text-slate-500">
                    <strong class="font-semibold text-blue-600 before:content-['—_']">
                        { "Sohibjon Orziqulov" }
                    </strong>
                    { ", Uzinfocom da Open Source Team Lead" }
                </figcaption>
            </a>
        </figure>
    }
}

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <header class="overflow-hidden bg-slate-100 lg:bg-transparent lg:px-5">
            <div class="mx-auto grid max-w-6xl grid-cols-1 grid-rows-[auto_1fr] gap-y-16 pt-16 md:pt-20 lg:grid-cols-12 lg:gap-y-20 lg:px-3 lg:pb-36 lg:pt-20 xl:py-32">
                <div class="relative flex items-end lg:col-span-5 lg:row-span-2">
                    <div class="absolute -bottom-12 -top-20 left-0 right-1/2 z-10 rounded-br-6xl bg-orange-500 text-white/10 md:bottom-8 lg:-inset-y-32 lg:left-[-100vw] lg:right-full lg:-mr-40">
                        <GridPattern
                            x="100%"
                            y="100%"
                            pattern_transform="translate(112 64)" />
                    </div>
                    <div class="relative z-10 mx-auto flex w-64 rounded-xl bg-slate-600 shadow-xl md:w-80 lg:w-auto">
                        <img class="w-full" src="images/cover.jpg" alt="" />
                    </div>
                </div>
                <div class="relative px-4 sm:px-6 lg:col-span-7 lg:pb-14 lg:pl-16 lg:pr-0 xl:pl-20">
                    <div class="hidden lg:absolute lg:-top-32 lg:bottom-0 lg:left-[-100vw] lg:right-[-100vw] lg:block lg:bg-slate-100" />
                    <Testimonial />
                </div>
                <div class="bg-white pt-16 lg:col-span-7 lg:bg-transparent lg:pl-16 lg:pt-0 xl:pl-20">
                    <div class="mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4 lg:px-0">
                        <h1 class="font-display text-5xl font-extrabold text-slate-900 sm:text-6xl">
                            { "Rust Dasturlash Tili" }
                        </h1>
                        <p class="mt-4 text-3xl text-slate-600">
                            { "Hammaga ishonchli va samarali dastur yaratishga imkon beruvchi
                            dasturlash tili." }
                        </p>
                        <div class="mt-8 flex gap-4">
                            <Button href="https://t.me/rustlanguz"
                                color="blue"
                                variant="solid"
                            >
                                { "Guruhga qo'shilish" }
                            </Button>
                            <Button
                                href="https://doc.rust-lang.uz/book/"
                                variant="outline"
                                color="blue"
                            >
                                {"O'rganish"}
                            </Button>
                            <Button
                                href="https://doc.rust-lang.uz/rust-by-example/"
                                variant="outline"
                                color="blue"
                            >
                                {"Misollar yordamida"}
                            </Button>
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}
