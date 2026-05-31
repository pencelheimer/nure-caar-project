import { createI18n } from 'vue-i18n'
import en from './en'
import uk from './uk'

const LOCALE_KEY = 'locale'
const saved      = localStorage.getItem(LOCALE_KEY)
const browser    = navigator.language.startsWith('uk') ? 'uk' : 'en'
const locale     = saved ?? browser

document.documentElement.lang = locale

export const i18n = createI18n({
  legacy: false,
  locale,
  fallbackLocale: 'en',
  messages: { en, uk },
})

export function setLocale(lang) {
  i18n.global.locale.value    = lang
  document.documentElement.lang = lang
  localStorage.setItem(LOCALE_KEY, lang)
}
