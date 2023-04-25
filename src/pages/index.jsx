import Head from 'next/head'

import { Contributor } from '@/components/Contributor'
import { Footer } from '@/components/Footer'
import { Socials } from '@/components/Socials'
import { Hero } from '@/components/Hero'
import { Introduction } from '@/components/Introduction'
import { NavBar } from '@/components/NavBar'
import { Communities } from '@/components/Communities'
import { Resources } from '@/components/Resources'
import { Projects } from '@/components/Projects'

export default function Home() {
  return (
    <>
      <Head>
        <title>Rust Dasturlash Tili - O'zbek Jamiyati</title>
        <meta
          name="description"
          content="Rust Dasturlash Tili - O'zbek Jamiyati"
        />
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
  )
}
