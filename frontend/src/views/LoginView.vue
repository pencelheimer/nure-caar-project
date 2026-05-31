<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore }  from '../stores/auth'
import { useThemeStore } from '../stores/theme'
import { i18n, setLocale } from '../i18n'
import EyeIcon from '../components/EyeIcon.vue'

const { t } = useI18n()
const router = useRouter()
const auth   = useAuthStore()

const mode      = ref('login')
const email     = ref('')
const password  = ref('')
const firstName = ref('')
const lastName  = ref('')
const loading   = ref(false)
const error     = ref('')

const isRegister    = computed(() => mode.value === 'register')
const currentLocale = computed(() => i18n.global.locale.value)
const theme = useThemeStore()

const themeOptions = [
  { value: 'system', icon: '⬡' },
  { value: 'light',  icon: '☀' },
  { value: 'dark',   icon: '☾' },
]

const showPassword = ref(false)

function switchMode(m) {
  mode.value  = m
  error.value = ''
}

async function submit() {
  error.value = ''
  if (!email.value || !password.value) {
    error.value = t('auth.fillAllFields')
    return
  }
  loading.value = true
  try {
    if (isRegister.value) {
      await auth.register({
        email: email.value,
        password: password.value,
        first_name: firstName.value || undefined,
        last_name:  lastName.value  || undefined,
      })
    } else {
      await auth.login(email.value, password.value)
    }
    router.push(auth.isAdmin ? '/admin/dashboard' : '/app/reservoirs')
  } catch (e) {
    error.value = e?.message || t(isRegister.value ? 'auth.registrationFailed' : 'auth.invalidCredentials')
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login-page">
    <div class="login-card card">
      <div class="login-header">
        <span class="logo">💧</span>
        <h1>SmartTank</h1>
      </div>

      <div class="controls-bar">
        <div class="theme-switcher">
          <button
            v-for="opt in themeOptions"
            :key="opt.value"
            class="ctrl-btn"
            :class="{ active: theme.preference === opt.value }"
            @click="theme.setPreference(opt.value)"
          >{{ opt.icon }}</button>
        </div>
        <div class="lang-switcher">
          <button class="ctrl-btn" :class="{ active: currentLocale === 'uk' }" @click="setLocale('uk')">UK</button>
          <button class="ctrl-btn" :class="{ active: currentLocale === 'en' }" @click="setLocale('en')">EN</button>
        </div>
      </div>

      <div class="tabs">
        <button class="tab" :class="{ active: mode === 'login' }" @click="switchMode('login')">
          {{ t('auth.signIn') }}
        </button>
        <button class="tab" :class="{ active: mode === 'register' }" @click="switchMode('register')">
          {{ t('auth.register') }}
        </button>
      </div>

      <form class="form" @submit.prevent="submit">
        <template v-if="isRegister">
          <div class="name-row">
            <div class="form-group">
              <label class="form-label">{{ t('auth.firstName') }}</label>
              <input v-model="firstName" type="text" class="form-input" autocomplete="given-name" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ t('auth.lastName') }}</label>
              <input v-model="lastName" type="text" class="form-input" autocomplete="family-name" />
            </div>
          </div>
        </template>

        <div class="form-group">
          <label class="form-label">{{ t('auth.email') }}</label>
          <input v-model="email" type="email" class="form-input" placeholder="you@example.com" autocomplete="email" />
        </div>
        <div class="form-group">
          <label class="form-label">{{ t('auth.password') }}</label>
          <div class="input-wrap">
            <input
              v-model="password"
              :type="showPassword ? 'text' : 'password'"
              class="form-input"
              placeholder="••••••••"
              :autocomplete="isRegister ? 'new-password' : 'current-password'"
            />
            <button type="button" class="eye-btn" @click="showPassword = !showPassword">
              <EyeIcon :open="!showPassword" />
            </button>
          </div>
        </div>

        <p v-if="error" class="form-error">{{ error }}</p>

        <button type="submit" class="btn btn-primary submit-btn" :disabled="loading">
          {{ loading ? t('common.loading') : isRegister ? t('auth.createAccount') : t('auth.signIn') }}
        </button>
      </form>
    </div>
  </div>
</template>

<style scoped>
.login-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
}
.login-card {
  width: 100%;
  max-width: 400px;
  padding: 36px 32px;
}
.login-header { text-align: center; margin-bottom: 16px; }
.logo { font-size: 40px; display: block; margin-bottom: 8px; }
.login-header h1 { font-size: 22px; font-weight: 700; }

.controls-bar {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 8px;
  margin-bottom: 20px;
}
.theme-switcher,
.lang-switcher {
  display: flex;
  gap: 2px;
  background: var(--surface-alt);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  padding: 2px;
}
.ctrl-btn {
  padding: 3px 8px;
  border-radius: 3px;
  border: none;
  background: none;
  color: var(--text-muted);
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  font-family: inherit;
  line-height: 1;
  transition: background 0.15s, color 0.15s;
}
.ctrl-btn:hover { background: var(--border); color: var(--text); }
.ctrl-btn.active { background: var(--primary); color: #fff; }

.tabs {
  display: flex;
  border-bottom: 1px solid var(--border);
  margin-bottom: 24px;
}
.tab {
  flex: 1;
  padding: 10px;
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-muted);
  cursor: pointer;
  margin-bottom: -1px;
  transition: color 0.15s, border-color 0.15s;
  font-family: inherit;
}
.tab.active { color: var(--primary); border-bottom-color: var(--primary); }

.form { display: flex; flex-direction: column; gap: 16px; }
.name-row { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
.submit-btn { width: 100%; justify-content: center; }

.input-wrap { position: relative; }
.input-wrap .form-input { padding-right: 40px; width: 100%; }
.eye-btn {
  position: absolute;
  right: 10px;
  top: 50%;
  transform: translateY(-50%);
  background: none;
  border: none;
  cursor: pointer;
  padding: 2px;
  display: flex;
  align-items: center;
  color: var(--text-muted);
  transition: color 0.15s;
}
.eye-btn:hover { color: var(--text); }
</style>
