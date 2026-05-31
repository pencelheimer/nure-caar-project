<script setup>
import { computed } from 'vue'
import { RouterLink, RouterView, useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '../stores/auth'
import { useThemeStore } from '../stores/theme'
import { i18n, setLocale } from '../i18n'

const { t } = useI18n()
const auth   = useAuthStore()
const theme  = useThemeStore()
const router = useRouter()
const route  = useRoute()

const adminNav = computed(() => [
  { path: '/admin/dashboard',  label: t('nav.dashboard'),  icon: '⊞' },
  { path: '/admin/users',      label: t('nav.users'),       icon: '👥' },
  { path: '/admin/audit-logs', label: t('nav.auditLogs'),  icon: '📋' },
])

const userNav = computed(() => [
  { path: '/app/reservoirs', label: t('nav.reservoirs'), icon: '🛢️' },
  { path: '/app/devices',    label: t('nav.devices'),    icon: '📡' },
  { path: '/app/alerts',     label: t('nav.alerts'),     icon: '🔔' },
])

const isAdminSection = computed(() => route.path.startsWith('/admin'))

const nav      = computed(() => isAdminSection.value ? adminNav.value : userNav.value)
const subtitle = computed(() => isAdminSection.value ? t('nav.adminPanel') : t('nav.monitoring'))

const displayName = computed(() =>
  [auth.user?.first_name, auth.user?.last_name].filter(Boolean).join(' ') || auth.user?.email
)

const currentLocale = computed(() => i18n.global.locale.value)

const themeOptions = [
  { value: 'system', icon: '⬡', title: 'System' },
  { value: 'light',  icon: '☀', title: 'Light' },
  { value: 'dark',   icon: '☾', title: 'Dark' },
]

function isActive(path) {
  return route.path.startsWith(path)
}

function logout() {
  auth.logout()
  router.push('/login')
}
</script>

<template>
  <div class="layout">
    <aside class="sidebar">
      <div class="sidebar-header">
        <span class="sidebar-logo">💧</span>
        <div class="brand-block">
          <div class="brand">SmartTank</div>
          <div class="brand-sub">{{ subtitle }}</div>
        </div>
        <RouterLink
          v-if="auth.isAdmin"
          :to="isAdminSection ? '/app/reservoirs' : '/admin/dashboard'"
          class="view-switch"
          :title="isAdminSection ? t('nav.switchToUser') : t('nav.switchToAdmin')"
        >{{ isAdminSection ? '👤' : '🛡' }}</RouterLink>
      </div>

      <nav class="sidebar-nav">
        <RouterLink
          v-for="item in nav"
          :key="item.path"
          :to="item.path"
          class="nav-link"
          :class="{ active: isActive(item.path) }"
        >
          <span class="nav-icon">{{ item.icon }}</span>
          {{ item.label }}
        </RouterLink>
      </nav>

      <div class="sidebar-footer">
        <RouterLink to="/app/profile" class="sidebar-user" :class="{ active: isActive('/app/profile') }">
          <div class="user-name">{{ displayName }}</div>
          <div class="user-email">{{ auth.user?.email }}</div>
        </RouterLink>

        <div class="theme-switcher">
          <button
            v-for="opt in themeOptions"
            :key="opt.value"
            class="theme-btn"
            :class="{ active: theme.preference === opt.value }"
            :title="opt.title"
            @click="theme.setPreference(opt.value)"
          >{{ opt.icon }}</button>
        </div>
        <div class="lang-switcher">
          <button
            class="lang-btn"
            :class="{ active: currentLocale === 'uk' }"
            @click="setLocale('uk')"
          >UK</button>
          <button
            class="lang-btn"
            :class="{ active: currentLocale === 'en' }"
            @click="setLocale('en')"
          >EN</button>
        </div>

        <button class="btn btn-ghost btn-sm logout-btn" @click="logout">
          {{ t('common.signOut') }}
        </button>
      </div>
    </aside>

    <main class="main">
      <RouterView />
    </main>
  </div>
</template>

<style scoped>
.layout {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

.sidebar {
  width: 220px;
  flex-shrink: 0;
  background: var(--sidebar);
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 20px 16px;
  border-bottom: 1px solid #1e293b;
}
.sidebar-logo { font-size: 28px; }
.brand-block  { flex: 1; min-width: 0; }
.brand        { color: #f1f5f9; font-size: 15px; font-weight: 700; }
.brand-sub    { color: var(--sidebar-text); font-size: 11px; }

.view-switch {
  font-size: 18px;
  line-height: 1;
  padding: 4px;
  border-radius: var(--radius-sm);
  color: var(--sidebar-text);
  transition: background 0.15s, color 0.15s;
  flex-shrink: 0;
}
.view-switch:hover { background: var(--sidebar-hover); color: var(--sidebar-active); }

.sidebar-nav {
  flex: 1;
  padding: 12px 8px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.nav-link {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 9px 12px;
  border-radius: var(--radius-sm);
  color: var(--sidebar-text);
  font-size: 13px;
  font-weight: 500;
  transition: background 0.15s, color 0.15s;
}
.nav-link:hover,
.nav-link.active { background: var(--sidebar-hover); color: var(--sidebar-active); }
.nav-link.active  { color: #60a5fa; }
.nav-icon { font-size: 16px; width: 20px; text-align: center; }

.sidebar-footer {
  padding: 16px;
  border-top: 1px solid #1e293b;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.sidebar-user {
  display: block;
  padding: 8px 10px;
  border-radius: var(--radius-sm);
  text-decoration: none;
  transition: background 0.15s;
  margin: 0 -2px;
  min-width: 0;
}
.sidebar-user:hover,
.sidebar-user.active { background: var(--sidebar-hover); }
.user-name  { color: #f1f5f9; font-size: 13px; font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.user-email { color: var(--sidebar-text); font-size: 11px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }

.theme-switcher,
.lang-switcher {
  display: flex;
  background: #0a0f1e;
  border-radius: var(--radius-sm);
  padding: 2px;
  gap: 2px;
}

.theme-btn,
.lang-btn {
  flex: 1;
  background: none;
  border: none;
  color: var(--sidebar-text);
  font-size: 13px;
  padding: 5px 0;
  border-radius: 3px;
  cursor: pointer;
  transition: background 0.15s, color 0.15s;
  line-height: 1;
  font-family: inherit;
  font-weight: 500;
}
.theme-btn { font-size: 14px; }
.theme-btn:hover, .theme-btn.active,
.lang-btn:hover,  .lang-btn.active {
  background: var(--sidebar-hover);
  color: var(--sidebar-active);
}
.theme-btn.active, .lang-btn.active { color: #60a5fa; }

.logout-btn { width: 100%; justify-content: center; color: var(--sidebar-text); }

.main {
  flex: 1;
  padding: 28px 32px;
  overflow-y: auto;
}
</style>
