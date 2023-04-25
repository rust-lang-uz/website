import { Container } from '@/components/Container'
import { Expandable } from '@/components/Expandable'
import { SectionHeading } from '@/components/SectionHeading'

const projects = {
  'Buyruq satri': {
    'Sodda va tez': 'https://www.rust-lang.org/learn',
    'Oson ishga tushuvchan':
      'https://rust-cli.github.io/book/tutorial/packaging.html',
    'Tushunarli konfiguratsiya':
      'https://rust-cli.github.io/book/in-depth/config-files.html',
    "Qo'llanma? Tayyor": 'https://rust-cli.github.io/book/in-depth/docs.html',
    'Oson logginglar':
      'https://rust-cli.github.io/book/in-depth/human-communication.html',
  },
  Webassembly: {
    'Bashoratli tezlik': 'https://webassembly.org/',
    'Kichik dastur hajmi': 'https://rustwasm.github.io/docs/book',
    'Zamonaviy qulayliklar':
      'https://developer.mozilla.org/en-US/docs/WebAssembly',
  },
  "Tarmog'lar": {
    'Kam iz': 'https://docs.rs/reqwest/',
    'Xavfsiz va ishonchli': 'https://rocket.rs/',
    "O'suvchan konkurrensiya": 'https://www.rust-lang.org/what/networking',
  },
  Embed: {
    'Kuchli statistik tahlil':
      'https://docs.rust-embedded.org/book/static-guarantees/',
    'Moslashuvchan xotira': 'https://docs.rust-embedded.org/book/collections/',
    "Qo'rquvsiz muvofiqlik": 'https://docs.rust-embedded.org/book/concurrency/',
  },
}

export function Projects() {
  return (
    <section
      id="projects"
      aria-labelledby="projects-title"
      className="scroll-mt-14 py-16 sm:scroll-mt-32 sm:py-20 lg:py-32"
    >
      <Container>
        <SectionHeading number="1" id="table-of-contents-title">
          Loyihalar
        </SectionHeading>
        <p className="mt-8 font-display text-4xl font-bold tracking-tight text-slate-900">
          Rust qamrab oluvchi yo'nalishlar
        </p>
        <p className="mt-4 text-lg tracking-tight text-slate-700">
          Rust dasturlash tilida bir necha yo'nalishlar bo'yicha loyiha va
          dasturlar yozsa bo'ladi. Ushbu keltirilgan ro'yxatda ushbu
          yo'nalishlar ko'rishingiz mumkin.
        </p>
        <Expandable>
          {({ isExpanded }) => (
            <>
              <ol role="list" className="mt-16 space-y-10 sm:space-y-16">
                {Object.entries(projects)
                  .slice(0, isExpanded ? undefined : 2)
                  .map(([title, pages]) => (
                    <li key={title}>
                      <h3 className="font-display text-3xl font-bold tracking-tight text-slate-900">
                        {title}
                      </h3>
                      <ol
                        role="list"
                        className="mt-8 divide-y divide-slate-300/30 rounded-2xl bg-slate-50 px-6 py-3 text-base tracking-tight sm:px-8 sm:py-7"
                      >
                        {Object.entries(pages).map(([title, link]) => (
                          <li
                            key={title}
                            className="flex justify-between py-3"
                            aria-label={`${title} on page ${link}`}
                          >
                            <span
                              className="font-medium text-slate-900"
                              aria-hidden="true"
                            >
                              {title}
                            </span>
                            <span
                              className="font-mono text-slate-400"
                              aria-hidden="true"
                            >
                              <a href={link}>Ko'proq</a>
                            </span>
                          </li>
                        ))}
                      </ol>
                    </li>
                  ))}
              </ol>
              <Expandable.Button>Ko'proq ko'rsatish</Expandable.Button>
            </>
          )}
        </Expandable>
      </Container>
    </section>
  )
}
