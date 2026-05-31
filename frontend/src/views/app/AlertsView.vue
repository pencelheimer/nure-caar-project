<script setup>
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import * as alertApi     from '../../api/alerts'
import * as reservoirApi from '../../api/reservoirs'
import { useToastStore } from '../../stores/toast'
import Spinner from '../../components/Spinner.vue'

const { t }  = useI18n()
const toast  = useToastStore()

const alerts     = ref([])
const reservoirs = ref([])
const loading    = ref(true)
const loadingMore = ref(false)
const hasMore    = ref(false)

const LIMIT = 30
let offset  = 0

onMounted(async () => {
  try {
    const [resList] = await Promise.all([
      reservoirApi.getReservoirs(),
    ])
    reservoirs.value = resList
    await fetchPage()
  } catch {
    toast.error(t('alerts.failLoad'))
  } finally {
    loading.value = false
  }
})

async function fetchPage() {
  const page = await alertApi.getAlerts({ limit: LIMIT, offset })
  alerts.value.push(...page)
  hasMore.value = page.length === LIMIT
  offset += page.length
}

async function loadMore() {
  loadingMore.value = true
  try {
    await fetchPage()
  } catch {
    toast.error(t('alerts.failLoad'))
  } finally {
    loadingMore.value = false
  }
}

function reservoirName(id) {
  return reservoirs.value.find(r => r.id === id)?.name ?? `#${id}`
}

function conditionLabel(type, threshold) {
  const word = { greater_than: t('alerts.above'), less_than: t('alerts.below'), equals: t('alerts.equals') }[type] ?? type
  return `${word} ${threshold} L`
}

function fmtDate(dt) {
  return dt ? new Date(dt).toLocaleString() : '—'
}
</script>

<template>
  <div>
    <div class="page-header">
      <h1 class="page-title">{{ t('alerts.title') }}</h1>
    </div>

    <Spinner v-if="loading" />

    <div v-else-if="alerts.length === 0" class="empty-state">
      <p class="text-muted">{{ t('alerts.none') }}</p>
    </div>

    <template v-else>
      <div class="card" style="padding:0;overflow:hidden">
        <table class="table">
          <thead>
            <tr>
              <th>{{ t('alerts.colTime') }}</th>
              <th>{{ t('alerts.colReservoir') }}</th>
              <th>{{ t('alerts.colCondition') }}</th>
              <th>{{ t('alerts.colValue') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="a in alerts" :key="a.id">
              <td class="text-muted" style="font-size:13px;white-space:nowrap">{{ fmtDate(a.triggered_at) }}</td>
              <td style="font-weight:500">{{ reservoirName(a.reservoir_id) }}</td>
              <td class="text-muted" style="font-size:13px">{{ conditionLabel(a.condition_type, a.threshold) }}</td>
              <td style="font-weight:500">{{ a.value.toFixed(1) }} л</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div v-if="hasMore" style="text-align:center;margin-top:16px">
        <button class="btn btn-outline btn-sm" :disabled="loadingMore" @click="loadMore">
          {{ loadingMore ? t('common.loading') : t('common.loadMore') }}
        </button>
      </div>
    </template>
  </div>
</template>

<style scoped>
.empty-state { text-align: center; padding: 60px; }
</style>
