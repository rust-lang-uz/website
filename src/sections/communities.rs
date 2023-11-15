use crate::functions::*;
use yew::prelude::*;

struct Community {
    title: &'static str,
    link: &'static str,
    image_src: &'static str,
    bg_class: &'static str,
}

const COMMUNITIES: [Community; 3] = [
    Community {
        title: "GitHub",
        link: "https://github.com/rust-lang-uz",
        image_src: "/images/communities/github.svg",
        bg_class: "bg-[radial-gradient(#2C313D_35%,#000)]",
    },
    Community {
        title: "Telegram",
        link: "https://t.me/rustlanguz",
        image_src: "/images/communities/telegram.svg",
        bg_class: "bg-[#0088CC]",
    },
    Community {
        title: "Discord",
        link: "https://discord.com/invite/rust-lang",
        image_src: "/images/communities/discord.svg",
        bg_class: "bg-[#6366F1]",
    },
];

#[function_component(Communities)]
pub fn communities() -> Html {
    html! {
        <section
            id="communities"
            aria-labelledby="communities-title"
            class="scroll-mt-14 py-16 sm:scroll-mt-32 sm:py-20 lg:py-32"
        >
            <Container>
                <SectionHeading number="3" id="communities-title">
                    {"Jamiyatlar"}
                </SectionHeading>
                <p class="mt-8 font-display text-4xl font-bold tracking-tight text-slate-900">
                    {"Siz hech qachon yolg'iz emassiz!"}
                </p>
                <p class="mt-4 text-lg tracking-tight text-slate-700">
                    {"Rust dasturlash tilini o'rganishning qiziqarli olamida sizning kelishingizni iliq va tarbiyali hamjamiyat kutmoqda. Rust ishqibozlari bilan bog'lanish, yo'l-yo'riq izlash va bilim almashish imkoniyatidan foydalaning. Hamkorlik ruhi va Rustni rivojlantirishga bo'lgan umumiy ishtiyoq orqali birlashgan Rust O'zbekiston hamjamiyatihga qo'shiling."}
                </p>
            </Container>
            <Container size="lg" class="mt-16">
                <ol
                    role="list"
                    class="-mx-3 grid grid-cols-1 gap-y-10 lg:grid-cols-3 lg:text-center xl:-mx-12 xl:divide-x xl:divide-slate-400/20"
                >
                    {
                        for COMMUNITIES.iter().map(|community| {
                            html! {
                                <a href={community.link}>
                                    <li
                                        class="grid auto-rows-min grid-cols-1 items-center gap-8 px-3 sm:grid-cols-2 sm:gap-y-10 lg:grid-cols-1 xl:px-12 rounded-2xl hover:bg-gray-200 p-4 m-2"
                                    >
                                        <div class="relative h-48 overflow-hidden rounded-2xl shadow-lg sm:h-60 lg:h-40">
                                            <div class={format!("absolute inset-0 flex items-center justify-center {}", community.bg_class)}>
                                                <img src={community.image_src} alt={community.title} />
                                            </div>
                                        </div>
                                        <div>
                                            <h3 class="text-base font-medium tracking-tight text-slate-900">
                                                {community.title}
                                            </h3>
                                        </div>
                                    </li>
                                </a>
                            }
                        })
                    }
                </ol>
            </Container>
        </section>
    }
}
