import Image from 'next/image'

import Link from 'next/link'
import { Container } from '@/components/Container'
import { SectionHeading } from '@/components/SectionHeading'
import discordImage from '@/images/communities/discord.svg'
import githubImage from '@/images/communities/github.svg'
import telegramImage from '@/images/communities/telegram.svg'

const communities = [
  {
    title: 'GitHub',
    link: 'https://github.com/rust-lang-uz',
    image: function GitHubImage() {
      return (
        <div className="absolute inset-0 flex items-center justify-center bg-[radial-gradient(#2C313D_35%,#000)]">
          <Image src={githubImage} alt="GitHub" unoptimized />
        </div>
      )
    },
  },
  {
    title: 'Telegram',
    link: 'https://t.me/rustlanguz',
    image: function TelegramImage() {
      return (
        <div className="absolute inset-0 flex items-center justify-center bg-[#0088CC]">
          <Image src={telegramImage} alt="Telegram" unoptimized />
        </div>
      )
    },
  },
  {
    title: 'Discord',
    link: 'https://discord.com/invite/rust-lang',
    image: function DiscordImage() {
      return (
        <div className="absolute inset-0 flex items-center justify-center bg-[#6366F1]">
          <Image src={discordImage} alt="Discord" unoptimized />
        </div>
      )
    },
  },
]

export function Communities() {
  return (
    <section
      id="communities"
      aria-labelledby="communities-title"
      className="scroll-mt-14 py-16 sm:scroll-mt-32 sm:py-20 lg:py-32"
    >
      <Container>
        <SectionHeading number="3" id="communities-title">
          Jamiyatlar
        </SectionHeading>
        <p className="mt-8 font-display text-4xl font-bold tracking-tight text-slate-900">
          Siz hech qachon yolg'iz emassiz!
        </p>
        <p className="mt-4 text-lg tracking-tight text-slate-700">
          Rust dasturlash tilini o'rganishning qiziqarli olamida sizning
          kelishingizni iliq va tarbiyali hamjamiyat kutmoqda. Rust ishqibozlari
          bilan bog'lanish, yo'l-yo'riq izlash va bilim almashish imkoniyatidan
          foydalaning. Hamkorlik ruhi va Rustni rivojlantirishga bo'lgan umumiy
          ishtiyoq orqali birlashgan Rust O'zbekiston hamjamiyatihga qo'shiling.
        </p>
      </Container>
      <Container size="lg" className="mt-16">
        <ol
          role="list"
          className="-mx-3 grid grid-cols-1 gap-y-10 lg:grid-cols-3 lg:text-center xl:-mx-12 xl:divide-x xl:divide-slate-400/20"
        >
          {communities.map((resource) => (
            <Link
              href={resource.link}
              key={resource.title.replaceAll(' ', '_')}
            >
              <li
                key={resource.title}
                className="grid auto-rows-min grid-cols-1 items-center gap-8 px-3 sm:grid-cols-2 sm:gap-y-10 lg:grid-cols-1 xl:px-12"
              >
                <div className="relative h-48 overflow-hidden rounded-2xl shadow-lg sm:h-60 lg:h-40">
                  <resource.image />
                </div>
                <div>
                  <h3 className="text-base font-medium tracking-tight text-slate-900">
                    {resource.title}
                  </h3>
                </div>
              </li>
            </Link>
          ))}
        </ol>
      </Container>
    </section>
  )
}
