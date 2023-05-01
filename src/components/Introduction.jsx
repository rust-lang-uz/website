import Link from 'next/link'

import { CheckIcon } from '@/components/CheckIcon'
import { Container } from '@/components/Container'

export function Introduction() {
  return (
    <section
      id="introduction"
      aria-label="Introduction"
      className="pb-16 pt-20 sm:pb-20 md:pt-36 lg:py-32"
    >
      <Container className="text-lg tracking-tight text-slate-700">
        <p className="font-display text-4xl font-bold tracking-tight text-slate-900">
          Nimaga Rust?
        </p>

        <p className="mt-8 font-display text-3xl font-bold tracking-tight text-slate-900">
          Tezlik
        </p>
        <p className="mt-4">
          Rust kuchini kashf eting: Tezlikni qabul qiling Texnik olamda inqilob
          qiladigan yashin tezligida dasturlash tili Rust bilan cheksiz
          imkoniyatlarni oching. Runtime yoki garbage collector - GClarning
          cheklovlaridan xoli, misli ko'rilmagan tezlik va resurslar
          samaradorligini his eting. Rust-ga sho'ng'ish bilan siz yuqori
          samarali xizmatlarni yaratish, embedded qurilmalarga hayot bag'ishlash
          va boshqa tillar bilan oson hamkorlik qilish uchun kuchni ochasiz.
          Kelajakni kashf qiling va Rust tezligidan foydalaning - bu shunchaki
          til emas, balki yanada samaraliroq, innovatsion va bog'langan dunyoga
          sayohat. Keling, birgalikda yangi cho'qqilarni zabt etaylik va mumkin
          bo'lgan chegaralarni qayta belgilaymiz.
        </p>

        <p className="mt-4 font-display text-3xl font-bold tracking-tight text-slate-900">
          Ishonchlilik
        </p>

        <p className="mt-4">
          Rust kuchini kashf eting: Ishonchlilikni qabul qiling Rust
          ishonchlilikka o'zining beqiyos e'tiborini jalb qiladi. Uning murakkab
          turdagi tizimi va xotirani boshqarishning innovatsion modeli
          kompilyatsiya vaqtida xavfsizlik va parallellikni ta'minlash uchun
          uyg'un ishlaydi. Barcha turdagi xatolarni kamaytiradigan, mustahkam va
          ishonchli ilovalar yaratish imkonini beruvchi tilda kodlash ishonchini
          his eting.
        </p>

        <p className="mt-4 font-display text-3xl font-bold tracking-tight text-slate-900">
          Samaradorlik
        </p>

        <p className="mt-4">
          Rust kuchini kashf eting: Samaradorlikni his qiling Rust kodlash
          tajribasini samaradorlikka e'tibor qaratgan holda oshiradi, puxta
          ishlab chiqilgan texnik hujjatlar va ko'p qirrali paket menejeri bilan
          to'ldirilgan foydalanuvchilarga qulay kompilyatorni taklif qiladi.
          Ko'plab amaliy vositalardan, jumladan, tushunarli xatolik xabarlariga
          ega do'stona kompilyator, jud ayaxshi ishlangan builder(quruvchi)
          intuitiv avtomatik to'ldiruvchi va turni tekshirish xususiyatiga ega
          dinamik tahrirlovchidan foydalangan holda tushunarli xato xabarlari
          orqali muammosiz navigatsiya qiling. Rust kodlash tajribangizni
          oshirish uchun mo'ljallangan va uni samarali va yoqimli qiladi.
        </p>
      </Container>
    </section>
  )
}
