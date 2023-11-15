use crate::functions::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TelegramIconProps {
    #[prop_or_default]
    class: Classes,
}

#[function_component(TelegramIcon)]
fn telegram_icon(props: &TelegramIconProps) -> Html {
    html! {
        <svg aria-hidden="true" viewBox="0 0 64 64" class={props.class.clone()}>
            <path d="M32,10c12.15,0,22,9.85,22,22s-9.85,22-22,22s-22-9.85-22-22S19.85,10,32,10z M39.589,40.968	c0.404-1.241,2.301-13.615,2.534-16.054c0.071-0.738-0.163-1.229-0.619-1.449c-0.553-0.265-1.371-0.133-2.322,0.21	c-1.303,0.47-17.958,7.541-18.92,7.951c-0.912,0.388-1.775,0.81-1.775,1.423c0,0.431,0.256,0.673,0.96,0.924	c0.732,0.261,2.577,0.82,3.668,1.121c1.05,0.29,2.243,0.038,2.913-0.378c0.709-0.441,8.901-5.921,9.488-6.402	c0.587-0.48,1.056,0.135,0.576,0.616c-0.48,0.48-6.102,5.937-6.844,6.693c-0.901,0.917-0.262,1.868,0.343,2.249	c0.689,0.435,5.649,3.761,6.396,4.295c0.747,0.534,1.504,0.776,2.198,0.776C38.879,42.942,39.244,42.028,39.589,40.968z" />
        </svg>
    }
}

#[function_component(Contributor)]
pub fn contributor() -> Html {
    html! {
        <section
            id="contributor"
            aria-labelledby="author-title"
            class="relative scroll-mt-14 pb-3 pt-8 sm:scroll-mt-32 sm:pb-16 sm:pt-10 lg:pt-16"
        >
            <div class="absolute inset-x-0 bottom-0 top-1/2 text-slate-900/10 [mask-image:linear-gradient(transparent,white)]">
                <GridPattern x="50%" y="100%" />
            </div>
            <div class="relative mx-auto max-w-5xl pt-16 sm:px-6">
                <div class="bg-slate-50 pt-px sm:rounded-6xl">
                    <div class="relative mx-auto -mt-16 h-44 w-44 overflow-hidden rounded-full bg-slate-200 md:float-right md:h-64 md:w-64 md:[shape-outside:circle(40%)] lg:mr-20 lg:h-72 lg:w-72">
                        <img
                            class="absolute inset-0 h-full w-full object-cover"
                            src="/images/avatars/contributor.jpg"
                            alt=""
                            sizes="(min-width: 1024px) 18rem, (min-width: 768px) 16rem, 11rem"
                        />
                    </div>
                    <div class="px-4 py-10 sm:px-10 sm:py-16 md:py-20 lg:px-20 lg:py-32">
                        <SectionHeading number="5" id="author-title">
                            { "Eng Faol Rustacean" }
                        </SectionHeading>
                        <p class="mt-8 font-display text-5xl font-extrabold tracking-tight text-slate-900 sm:text-6xl">
                            <span class="block text-blue-600">{"Otabek Ismoilov –"}</span>{" "}
                            { "Stabil dasturlar yaratish yanada oson!" }
                        </p>
                        <p class="mt-4 text-lg tracking-tight text-slate-700">
                            { "Tizimli dasturlash olamiga qiziqib, Rust dasturlash tiliga qiziqib o'rganishga kirishdim. Ushbu o'quv tajribasi davomida men uning ichki faoliyati va imkoniyatlari haqida qimmatli tushunchalarga ega bo'ldim. Rustni o'z loyihalarimda ishlatishni boshlaganimda, men uning ajoyib tezligi va samaradorligidan hayratda qoldim, bu mening mahsuldorligimni sezilarli darajada oshirdi. Rustning dasturlash hamjamiyatidagi imkoniyatlarini tushunib, Rustbookni o‘zbek tiliga tarjima qilib, o‘z hissamni qo‘shishga qaror qildim. Bu kengroq auditoriyaga Rust kuchiga kirish va tushunish imkonini beradi. Ishonchim komilki, Rustbook-ni tajriba darajasidan qat'i nazar, tizim dasturlashiga qiziqqan har bir kishiga tavsiya qilaman. Rust yordamida siz ham qiziqarli sayohatga chiqishingiz va dasturlash mahoratingizning haqiqiy imkoniyatlarini ochishingiz mumkin." }
                        </p>
                        <p class="mt-8">
                            <a href="https://t.me/Otabek_Ismoilov" class="inline-flex items-center text-base font-medium tracking-tight text-blue-600">
                                <TelegramIcon class="h-14 w-14 fill-current" />
                                <span class="ml-4">{"Telegram da obuna bo'lish"}</span>
                            </a>
                        </p>
                    </div>
                </div>

            </div>
        </section>
    }
}
