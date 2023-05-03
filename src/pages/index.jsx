import Head from "next/head";

import { Contributor } from "@/components/Contributor";
import { Footer } from "@/components/Footer";
import { Socials } from "@/components/Socials";
import { Hero } from "@/components/Hero";
import { Introduction } from "@/components/Introduction";
import { NavBar } from "@/components/NavBar";
import { Communities } from "@/components/Communities";
import { Resources } from "@/components/Resources";
import { Projects } from "@/components/Projects";

const image = "https://rust-lang.uz/og.png";
const title = "Rust Dasturlash Tili - O'zbek Jamiyati";
const description = "Rust dasturlash tiliga qaratilgan O'zbek hamjamiyati";

export default function Home() {
	return (
		<>
			<Head>
				{/* Title */}
				<title>{title}</title>
				<meta name="og:title" content={title} />

				{/* Description */}
				<meta name="description" content={description} />
				<meta name="og:description" content={description} />

				{/* Image */}
				<meta name="twitter:image" content={image} />
				<meta name="og:image" content={image} />

				{/* URL */}
				<meta name="og:url" content="https://rust-lang.uz" />

				{/* General */}
				<meta name="viewport" content="width=device-width, initial-scale=1.0" />
				<meta httpEquiv="Content-Language" content="uz" />
				<meta name="twitter:card" content="summary_large_image" />
				<meta name="twitter:site" content="@rust-lang" />
				<meta name="apple-mobile-web-app-title" content="Rust O'zbekiston" />
				<meta name="author" content="Rust O'zbekiston" />

				{/* Favicons */}
				<link rel="manifest" href="/manifest.json" />
				<meta name="theme-color" content="#000000" />
				<link
					rel="apple-touch-icon"
					sizes="180x180"
					href="/favicon-180-precomposed.png"
				/>
				<link rel="icon" type="image/png" href="/logo.png" />
				<link rel="alternate icon" type="image/svg+xml" href="/logo.svg" />
			</Head>
			<Hero />
			<Introduction />
			<NavBar />
			<Projects />
			<Resources />
			<Communities />
			<Socials />
			<Contributor />
			<Footer />
		</>
	);
}
