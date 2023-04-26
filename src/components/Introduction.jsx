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
          Rust o'ta tez va resurs tejamkor: hech qanday rantaym yoki axlat
          yig'uvchisiz (garbage collector - GC), tezlik muhim bo'lgan servislarni qo'llab
          quvvatlovchi, embedded qurilmalarda 
          ishlovchi va boshqa tillar bilan
          osongina biriktirish imkoniyatiga ega.
        </p>

        <p className="mt-4 font-display text-3xl font-bold tracking-tight text-slate-900">
          Ishonchlilik
        </p>

        <p className="mt-4">
          Rustning boy tip tizimi va egalik xotira modeli xavfsizligi va oqim
          xavfsizligi kompilyatsiya vaqti sizga har xil turdagi xatoliklardan
          holi bo'lish imkoniyatini beradi.
        </p>

        <p className="mt-4 font-display text-3xl font-bold tracking-tight text-slate-900">
          Samaradorlik
        </p>

        <p className="mt-4">
          Rustda yaxshi yozilgan dokumentatsiya, tushunarli xatolik xabarlariga
          ega do'stona kompilyator va paketlar menejeri va quruvchi, avto
          to'ldiruvchi va tiplar tekshiruvchi aqlli multi muharrir kabi eng
          foydali asboblar va shunga o'xshashlar mavjud.
        </p>
      </Container>
    </section>
  )
}
