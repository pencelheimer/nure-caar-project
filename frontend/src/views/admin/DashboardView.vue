<script setup>
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import * as adminApi from '../../api/admin'
import Spinner from '../../components/Spinner.vue'

const { t } = useI18n()

const stats   = ref(null)
const loading = ref(true)

const cards = computed(() => [
  { key: 'total_users',        label: t('dashboard.totalUsers'),        icon: '👥' },
  { key: 'total_reservoirs',   label: t('dashboard.reservoirs'),        icon: '🛢️' },
  { key: 'total_devices',      label: t('dashboard.devices'),           icon: '📡' },
  { key: 'alert_rules_active', label: t('dashboard.activeAlertRules'),  icon: '🔔' },
])

onMounted(async () => {
  try {
    stats.value = await adminApi.getStats()
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div>
    <div class="page-header">
      <h1 class="page-title">{{ t('dashboard.title') }}</h1>
    </div>
    <Spinner v-if="loading" />
    <div v-else-if="stats" class="stats-grid">
      <div v-for="card in cards" :key="card.key" class="card stat-card">
        <div class="stat-icon">{{ card.icon }}</div>
        <div class="stat-label">{{ card.label }}</div>
        <div class="stat-value">{{ stats[card.key] }}</div>
      </div>
    </div>
  </div>
</template>
