import { ref } from 'vue'
import { defineStore } from 'pinia'

let _id = 0

export const useToastStore = defineStore('toast', () => {
  const toasts = ref([])

  function show(message, type = 'info', duration = 4000) {
    const id = _id++
    toasts.value.push({ id, message, type })
    setTimeout(() => remove(id), duration)
  }

  function success(msg) { show(msg, 'success') }
  function error(msg)   { show(msg, 'error') }
  function info(msg)    { show(msg, 'info') }

  function remove(id) {
    toasts.value = toasts.value.filter(t => t.id !== id)
  }

  return { toasts, show, success, error, info, remove }
})
