use crate::functions::*;
use yew::prelude::*;

struct Video {
    title: &'static str,
    link: &'static str,
    description: &'static str,
    image: &'static str,
    runtime: (u32, u32), // (minutes, seconds)
}

const VIDEOS: &[Video] = &[
    Video {
        title: "Rust Crash Course",
		link: "https://www.youtube.com/watch?v=zF34dRivLOw",
		description:
			"Learn all the fundamentals of the Rust programming language in this crash course.",
		image: "https://i3.ytimg.com/vi/zF34dRivLOw/maxresdefault.jpg",
		runtime: (90, 43),
    },
    Video {
		title: "Rust 101 Crash Course",
		link: "https://www.youtube.com/watch?v=lzKeecy4OmQ",
		description:
			"You'll learn Rust from scratch and start your path to becoming a Rust Developer with this 6-hour course.",
		image: "https://i3.ytimg.com/vi/lzKeecy4OmQ/maxresdefault.jpg",
		runtime: (361, 14),
	},
	Video {
		title: "Rust Language 🦀 Crash Course for beginners",
		link: "https://www.youtube.com/watch?v=nweNM-TQYfs",
		description:
			"The first time I've been introduced to Rust was on January 2022, you might think \"oh that's pretty new\".",
		image: "https://i3.ytimg.com/vi/nweNM-TQYfs/maxresdefault.jpg",
		runtime: (60, 1)
	},
	Video {
		title: "Rust Programming Course for Beginners - Tutorial",
		link: "https://www.youtube.com/watch?v=MsocPEZBd-M",
		description:
			"Learn the Rust programming language in this course for beginners.",
		image: "https://i3.ytimg.com/vi/MsocPEZBd-M/maxresdefault.jpg",
		runtime: (85, 36),
	},
];

#[derive(Properties, PartialEq)]
pub struct PlayIconProps {
    #[prop_or_default]
    pub class: &'static str,
}

#[function_component(PlayIcon)]
fn play_icon(props: &PlayIconProps) -> Html {
    html! {
        <svg aria-hidden="true" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 16 16" class={props.class}>
            <path d="M6.75 10.25v-4.5L10.25 8l-3.5 2.25Z" />
            <circle cx="8" cy="8" r="6.25" fill="none" />
        </svg>
    }
}

#[function_component(Resources)]
pub fn resources() -> Html {
    html! {
        <section id="resources" aria-labelledby="screencasts-title" class="scroll-mt-14 py-16 sm:scroll-mt-32 sm:py-20 lg:py-32">
            <Container>
                <SectionHeading number="2" id="screencasts-title">
                    {"Resurslar"}
                </SectionHeading>
                <p class="mt-8 font-display text-4xl font-bold tracking-tight text-slate-900">
                    {"Hamma o'z bilimi va tajribasi bilan bo'lishishadi"}
                </p>
                <p class="mt-4 text-lg tracking-tight text-slate-700">
                    {"Rust hamjamiyati boshqa hamjamiyatlardan farqli, har bir rustacean hamjamiyat uchun biron kontent yaratish doirasida o'z bilim va tajribalari bilan bo'lishishga intilishadi."}
                </p>
            </Container>
            <Container size="lg" class="mt-16">
                <ol role="list" class="grid grid-cols-1 gap-x-8 gap-y-10 [counter-reset:video] sm:grid-cols-2 lg:grid-cols-4">
                    {
                        for VIDEOS.iter().map(|video| {
                            html! {
                                <a href={video.link}>
                                    <li class="[counter-increment:video] rounded-2xl hover:bg-gray-300 p-4">
                                        <div
                                            class="relative flex h-44 items-center justify-center rounded-2xl px-6 shadow-lg"
                                            style="background-image: conic-gradient(from -49.8deg at 50% 50%, #7331FF 0deg, #00A3FF 59.07deg, #4E51FF 185.61deg, #39DBFF 284.23deg, #B84FF1 329.41deg, #7331FF 360deg)"
                                        >
                                            <div class="flex overflow-hidden rounded shadow-sm">
                                                <img
                                                    src={video.image}
                                                    alt=""
                                                    width="200"
                                                    height="108"
                                                />
                                            </div>
                                            <div class="absolute bottom-2 left-2 flex items-center rounded-lg bg-black/30 px-1.5 py-0.5 text-sm text-white [@supports(backdrop-filter:blur(0))]:bg-white/10 [@supports(backdrop-filter:blur(0))]:backdrop-blur">
                                                <PlayIcon class="h-4 w-4 fill-current stroke-current" />
                                                <time
                                                    datetime={format!("{}m {}s", video.runtime.0, video.runtime.1)}
                                                    class="ml-2"
                                                >
                                                    {format!("{}:{:02}", video.runtime.0, video.runtime.1)}
                                                </time>
                                            </div>
                                        </div>
                                        <h3 class="mt-8 text-base font-medium tracking-tight text-slate-900 before:mb-2 before:block before:font-mono before:text-sm before:text-slate-500 before:content-[counter(video,decimal-leading-zero)]">
                                            {video.title}
                                        </h3>
                                        <p class="mt-2 text-sm text-slate-600">
                                            {video.description}
                                        </p>
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
