import { GridPattern } from '@/components/GridPattern'

export function Footer() {
  return (
    <footer className="relative pb-20 pt-5 sm:pb-32 sm:pt-14">
      <div className="absolute inset-x-0 top-0 h-32 text-slate-900/10 [mask-image:linear-gradient(white,transparent)]">
        <GridPattern x="50%" />
      </div>
      <div className="relative text-center text-sm text-slate-600">
        <p>
          CC0-1.0 Litsenziya ostida &copy;{' '}
          {new Date().getFullYear() !== 2023 ? '2023 - ' : ''}
          {new Date().getFullYear()} Rust O'zbek Hamjamiyati
        </p>
      </div>
    </footer>
  )
}
