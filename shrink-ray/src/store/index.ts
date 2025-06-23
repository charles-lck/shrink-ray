import { defineStore } from 'pinia';

export const useAppStore = defineStore('app', {
  state: () => ({
    lossless: true,
    replace: false,
    language: 'en',
    themeMode: 'light'
  }),
  actions: {
    changeLossless(type: boolean) {
      this.lossless = type
    },
    changeReplace(type: boolean) {
      this.replace = type
    },
    changeLanguage(type: string) {
      this.language = type
    }
  },
  getters: {

  },
  persist: true
});