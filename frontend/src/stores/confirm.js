import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useConfirmStore = defineStore('confirm', () => {
  const dialog = ref(null)

  function confirm({ title, message, confirmLabel = 'Confirm', danger = false }) {
    return new Promise(resolve => {
      dialog.value = { title, message, confirmLabel, danger, resolve }
    })
  }

  function answer(value) {
    dialog.value?.resolve(value)
    dialog.value = null
  }

  return { dialog, confirm, answer }
})
