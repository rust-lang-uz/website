import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'

export default function Home() {
  return (
    <div className={styles.container}>
      <Head>
        <title>Rust Uzbekistan!</title>
        <meta name="description" content="Generated by create next app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>
          <a href="https://rust-lang.org">Rust</a> O'zbek Jamiyatiga <br />
          Xush Kelibsiz!
        </h1>

        <p className={styles.description}>

          <code className={styles.code}>Hamjamiyat tomonidan to'plangan resurslar:</code>
        </p>

        <div className={styles.grid}>
          <a href="https://book.rust-lang.uz" className={styles.card}>
            <h2>Dokumentatsiya &rarr;</h2>
            <p>Rust haqida boshlang'ich ma'lumot va tajribalar beruvchi manba.</p>
          </a>

          <a href="https://alt.rust-lang.uz" className={styles.card}>
            <h2>Alternativ &rarr;</h2>
            <p>@ismoilovdev tomonidan yuritiladigan shaxsiy alternativ manba</p>
          </a>

          <a
            href="https://github.com/rust-lang-uz"
            className={styles.card}
          >
            <h2>Faoliyat &rarr;</h2>
            <p>GitHub dagi Ochiq Lavhalik dasturlarni rivojlantiruvchi organizatsiyamiz.</p>
          </a>

          <a
            href="https://t.me/rustlanguz"
            target="_blank"
            rel="noopener noreferrer"
            className={styles.card}
          >
            <h2>Hamjamiyat &rarr;</h2>
            <p>
              Telegram sotsial tarmog'ida joylashgan kommunikativ jamiyatimiz.
            </p>
          </a>
        </div>
      </main>

      <footer className={styles.footer}>
        <a
          href="https://vercel.com?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
          target="_blank"
          rel="noopener noreferrer"
        >
          Powered by{' '}
          <span className={styles.logo}>
            <Image src="/vercel.svg" alt="Vercel Logo" width={72} height={16} />
          </span>
        </a>
      </footer>
    </div>
  )
}
