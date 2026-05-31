<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import * as reservoirApi from '../../api/reservoirs'
import * as deviceApi    from '../../api/devices'
import { useToastStore } from '../../stores/toast'
import Spinner from '../../components/Spinner.vue'
import ModalDialog from '../../components/ModalDialog.vue'

const { t } = useI18n()
const router = useRouter()
const toast  = useToastStore()

const reservoirs = ref([])
const levels     = ref({}) // reservoirId -> { value, percent }
const loading    = ref(true)
const showModal  = ref(false)

const form   = ref({ name: '', description: '', capacity: '', location: '' })
const saving = ref(false)

onMounted(load)

async function load() {
  loading.value = true
  try {
    const [resList, devList] = await Promise.all([
      reservoirApi.getReservoirs(),
      deviceApi.getDevices(),
    ])
    reservoirs.value = resList

    const linked = devList.filter(d => d.reservoir_id != null)
    await Promise.all(linked.map(async d => {
      try {
        const ms = await deviceApi.getMeasurements(d.id, { limit: 1 })
        if (!ms.length) return
        const res = resList.find(r => r.id === d.reservoir_id)
        levels.value[d.reservoir_id] = {
          value:   ms[0].value,
          percent: res ? Math.min(100, (ms[0].value / res.capacity) * 100) : null,
        }
      } catch {}
    }))
  } catch {
    toast.error(t('reservoirs.failLoad'))
  } finally {
    loading.value = false
  }
}

function openCreate() {
  form.value = { name: '', description: '', capacity: '', location: '' }
  showModal.value = true
}

async function submit() {
  if (!form.value.name || !form.value.capacity) return
  saving.value = true
  try {
    const created = await reservoirApi.createReservoir({
      name:        form.value.name,
      description: form.value.description || null,
      capacity:    parseFloat(form.value.capacity),
      location:    form.value.location || null,
    })
    reservoirs.value.push(created)
    showModal.value = false
    toast.success(t('reservoirs.created'))
  } catch {
    toast.error(t('reservoirs.failCreate'))
  } finally {
    saving.value = false
  }
}

function levelColor(pct) {
  if (pct == null) return null
  if (pct >= 75) return 'var(--level-high)'
  if (pct >= 35) return 'var(--level-mid)'
  return 'var(--level-low)'
}
</script>

<template>
  <div>
    <div class="page-header">
      <h1 class="page-title">{{ t('reservoirs.title') }}</h1>
      <button class="btn btn-primary btn-sm" @click="openCreate">+ {{ t('reservoirs.add') }}</button>
    </div>

    <Spinner v-if="loading" />

    <div v-else-if="reservoirs.length === 0" class="empty-state">
      <p class="text-muted">{{ t('reservoirs.none') }}</p>
    </div>

    <div v-else class="grid">
      <div
        v-for="r in reservoirs"
        :key="r.id"
        class="card reservoir-card"
        @click="router.push(`/app/reservoirs/${r.id}`)"
      >
        <!-- background fill -->
        <div
          v-if="levels[r.id]?.percent != null"
          class="level-bg"
          :style="{
            height: `${levels[r.id].percent}%`,
            background: levelColor(levels[r.id].percent),
          }"
        />

        <div class="card-content">
          <div class="reservoir-name">{{ r.name }}</div>
          <div v-if="r.location" class="reservoir-location text-muted">📍 {{ r.location }}</div>

          <div class="reservoir-bottom">
            <div class="reservoir-capacity text-muted">
              {{ t('reservoirs.capacity') }}: <strong>{{ r.capacity }} L</strong>
            </div>
            <div v-if="levels[r.id]" class="level-info">
              <span class="level-value">{{ levels[r.id].value.toFixed(1) }} L</span>
              <span class="level-pct text-muted">{{ levels[r.id].percent?.toFixed(0) }}%</span>
            </div>
            <div v-else class="text-muted" style="font-size:11px">{{ t('reservoirs.noDevice') }}</div>
          </div>
        </div>
      </div>
    </div>

    <ModalDialog v-model="showModal" :title="t('reservoirs.addTitle')">
      <div class="form-group">
        <label class="form-label">{{ t('reservoirs.name') }}</label>
        <input v-model="form.name" type="text" class="form-input" />
      </div>
      <div class="form-group">
        <label class="form-label">{{ t('reservoirs.capacity') }}</label>
        <input v-model="form.capacity" type="number" min="1" class="form-input" />
      </div>
      <div class="form-group">
        <label class="form-label">{{ t('reservoirs.location') }} <span class="text-muted">({{ t('reservoirs.optional') }})</span></label>
        <input v-model="form.location" type="text" class="form-input" />
      </div>
      <div class="form-group">
        <label class="form-label">{{ t('reservoirs.description') }} <span class="text-muted">({{ t('reservoirs.optional') }})</span></label>
        <textarea v-model="form.description" class="form-input" rows="2" style="resize:vertical" />
      </div>
      <template #footer>
        <button class="btn btn-outline" @click="showModal = false">{{ t('common.cancel') }}</button>
        <button class="btn btn-primary" :disabled="saving || !form.name || !form.capacity" @click="submit">
          {{ saving ? t('common.loading') : t('reservoirs.add') }}
        </button>
      </template>
    </ModalDialog>
  </div>
</template>

<style scoped>
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 16px;
}

.reservoir-card {
  padding: 0;
  cursor: pointer;
  transition: box-shadow 0.15s, transform 0.15s;
  position: relative;
  overflow: hidden;
  min-height: 130px;
}
.reservoir-card:hover {
  box-shadow: 0 4px 16px rgba(0,0,0,0.12);
  transform: translateY(-1px);
}

.level-bg {
  position: absolute;
  bottom: 0; left: 0; right: 0;
  transition: height 0.8s cubic-bezier(0.4, 0, 0.2, 1);
  pointer-events: none;
}

.card-content {
  position: relative;
  z-index: 1;
  padding: 18px 20px;
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 4px;
}

.reservoir-name     { font-size: 15px; font-weight: 600; margin-bottom: 2px; }
.reservoir-location { font-size: 12px; }

.reservoir-bottom {
  margin-top: auto;
  padding-top: 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.reservoir-capacity { font-size: 12px; }

.level-info {
  display: flex;
  align-items: baseline;
  gap: 6px;
}
.level-value { font-size: 20px; font-weight: 700; }
.level-pct   { font-size: 12px; }

.empty-state { text-align: center; padding: 60px; }
</style>
