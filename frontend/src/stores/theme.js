import { ref, computed, watch } from 'vue'
import { defineStore } from 'pinia'

const PREF_KEY = 'theme-preference'
const media = window.matchMedia('(prefers-color-scheme: dark)')

export const useThemeStore = defineStore('theme', () => {
  const preference = ref(localStorage.getItem(PREF_KEY) ?? 'system')
  const systemDark = ref(media.matches)

  media.addEventListener('change', e => { systemDark.value = e.matches })

  const effectiveTheme = computed(() =>
    preference.value === 'system'
      ? (systemDark.value ? 'dark' : 'light')
      : preference.value
  )

  function setPreference(val) {
    preference.value = val
    localStorage.setItem(PREF_KEY, val)
  }

  watch(effectiveTheme, theme => {
    document.documentElement.setAttribute('data-theme', theme)
  }, { immediate: true })

  return { preference, effectiveTheme, setPreference }
})
