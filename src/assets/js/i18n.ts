// src/i18n.js
import { createI18n } from 'vue-i18n'
import en from './locales/en.json'
import zh_CN from './locales/zh_CN.json'
import zh_TW from './locales/zh_TW.json'
import ja from './locales/jp.json'
import ko from './locales/ko.json'
import es from './locales/es.json'
import fr from './locales/fr.json'

const messages = {
  en,
  zh_CN,
  zh_TW,
  ja,
  ko,
  es,
  fr
}

const i18n = createI18n({
  locale: 'en', // 默认语言
  fallbackLocale: 'en', // 回退语言
  messages
})

export default i18n