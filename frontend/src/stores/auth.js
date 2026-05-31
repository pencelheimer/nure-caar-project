import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import * as authApi from '../api/auth'

export const useAuthStore = defineStore('auth', () => {
  const token = ref(localStorage.getItem('token') || null)
  const user = ref(null)

  const isAuthenticated = computed(() => !!token.value)
  const isAdmin = computed(() => user.value?.role === 'admin')

  function setToken(t) {
    token.value = t
    if (t) localStorage.setItem('token', t)
    else localStorage.removeItem('token')
  }

  async function login(email, password) {
    const data = await authApi.login(email, password)
    setToken(data.token)
    await fetchMe()
  }

  async function register(payload) {
    const data = await authApi.register(payload)
    setToken(data.token)
    await fetchMe()
  }

  async function fetchMe() {
    if (!token.value) return
    try {
      user.value = await authApi.me()
    } catch {
      logout()
    }
  }

  function logout() {
    setToken(null)
    user.value = null
  }

  return { token, user, isAuthenticated, isAdmin, login, register, fetchMe, logout }
})
