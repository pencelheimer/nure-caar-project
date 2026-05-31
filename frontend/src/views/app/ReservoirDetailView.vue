<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import * as reservoirApi from '../../api/reservoirs'
import * as deviceApi from '../../api/devices'
import { useToastStore }   from '../../stores/toast'
import { useConfirmStore } from '../../stores/confirm'
import Spinner     from '../../components/Spinner.vue'
import LineChart   from '../../components/LineChart.vue'
import ModalDialog from '../../components/ModalDialog.vue'

const { t }   = useI18n()
const route   = useRoute()
const router  = useRouter()
const toast   = useToastStore()
const confirm = useConfirmStore()

const id = route.params.id

const reservoir    = ref(null)
const device       = ref(null)
const measurements = ref([])
const rules        = ref([])
const loading      = ref(true)

const showEdit = ref(false)
const editForm = ref({})

const newRule  = ref({ condition_type: 'less_than', threshold: '' })
const addingRule = ref(false)

// API returns DESC (newest first), so index 0 is the latest
const latestMeasurement = computed(() =>
  measurements.value.length ? measurements.value[0] : null
)
// chart expects chronological order (oldest → newest)
const chartMeasurements = computed(() => [...measurements.value].reverse())
const levelPercent = computed(() => {
  if (!latestMeasurement.value || !reservoir.value?.capacity) return null
  return Math.min(100, (latestMeasurement.value.value / reservoir.value.capacity) * 100)
})

onMounted(async () => {
  try {
    const [res, devices, ruleList] = await Promise.all([
      reservoirApi.getReservoir(id),
      deviceApi.getDevices(),
      reservoirApi.getRules(id),
    ])
    reservoir.value = res
    rules.value     = ruleList
    device.value    = devices.find(d => d.reservoir_id === res.id) ?? null

    if (device.value) {
      measurements.value = await deviceApi.getMeasurements(device.value.id, { limit: 200 })
    }
  } catch {
    toast.error(t('reservoirs.failLoad'))
    router.push('/app/reservoirs')
  } finally {
    loading.value = false
  }
})

function openEdit() {
  editForm.value = {
    name:        reservoir.value.name,
    description: reservoir.value.description ?? '',
    capacity:    reservoir.value.capacity,
    location:    reservoir.value.location ?? '',
  }
  showEdit.value = true
}

async function saveEdit() {
  try {
    const updated = await reservoirApi.updateReservoir(id, {
      name:        editForm.value.name || null,
      description: editForm.value.description || null,
      capacity:    parseFloat(editForm.value.capacity) || null,
      location:    editForm.value.location || null,
    })
    Object.assign(reservoir.value, updated)
    showEdit.value = false
    toast.success(t('reservoirs.updated'))
  } catch {
    toast.error(t('reservoirs.failUpdate'))
  }
}

async function deleteReservoir() {
  const ok = await confirm.confirm({
    title:        t('reservoirs.deleteTitle'),
    message:      t('reservoirs.deleteMessage', { name: reservoir.value.name }),
    confirmLabel: t('common.delete'),
    danger: true,
  })
  if (!ok) return
  try {
    await reservoirApi.deleteReservoir(id)
    toast.success(t('reservoirs.deleted'))
    router.push('/app/reservoirs')
  } catch {
    toast.error(t('reservoirs.failDelete'))
  }
}

async function addRule() {
  if (!newRule.value.threshold) return
  addingRule.value = true
  try {
    const rule = await reservoirApi.createRule(id, {
      reservoir_id:   parseInt(id),
      condition_type: newRule.value.condition_type,
      threshold:      parseFloat(newRule.value.threshold),
    })
    rules.value.push(rule)
    newRule.value.threshold = ''
    toast.success(t('reservoirs.ruleCreated'))
  } catch {
    toast.error(t('reservoirs.failCreateRule'))
  } finally {
    addingRule.value = false
  }
}

async function toggleRule(rule) {
  try {
    const updated = await reservoirApi.updateRule(rule.id, { is_active: !rule.is_active })
    Object.assign(rule, updated)
  } catch {
    toast.error(t('reservoirs.failRules'))
  }
}

async function removeRule(rule) {
  try {
    await reservoirApi.deleteRule(rule.id)
    rules.value = rules.value.filter(r => r.id !== rule.id)
    toast.success(t('reservoirs.ruleDeleted'))
  } catch {
    toast.error(t('reservoirs.failDeleteRule'))
  }
}

function conditionLabel(type) {
  return { greater_than: t('reservoirs.conditionAbove'), less_than: t('reservoirs.conditionBelow'), equals: t('reservoirs.conditionEquals') }[type] ?? type
}

function fmtDate(dt) {
  return dt ? new Date(dt).toLocaleString() : t('devices.never')
}
</script>

<template>
  <div class="page-root">
    <button class="btn btn-ghost btn-sm back-btn" @click="router.back()">
      ← {{ t('common.back') }}
    </button>

    <Spinner v-if="loading" />

    <div v-else-if="reservoir" class="reservoir-content">
      <div class="page-header">
        <h1 class="page-title">{{ reservoir.name }}</h1>
        <div style="display:flex;gap:8px">
          <button class="btn btn-outline btn-sm" @click="openEdit">{{ t('common.edit') }}</button>
          <button class="btn btn-danger btn-sm" @click="deleteReservoir">{{ t('common.delete') }}</button>
        </div>
      </div>

      <div class="detail-grid">
        <!-- Info -->
        <div class="card section">
          <div class="section-title">{{ t('reservoirs.name') }}</div>
          <dl class="info">
            <dt>{{ t('reservoirs.capacity') }}</dt>  <dd>{{ reservoir.capacity }} L</dd>
            <dt v-if="reservoir.location">{{ t('reservoirs.location') }}</dt>
            <dd v-if="reservoir.location">{{ reservoir.location }}</dd>
            <dt v-if="reservoir.description">{{ t('reservoirs.description') }}</dt>
            <dd v-if="reservoir.description">{{ reservoir.description }}</dd>
            <dt>{{ t('devices.colStatus') }}</dt>
            <dd>
              <span v-if="device" :class="`badge badge-${device.status === 'online' ? 'success' : 'danger'}`">
                {{ device.status === 'online' ? t('devices.online') : t('devices.offline') }}
              </span>
              <span v-else class="text-muted">{{ t('reservoirs.noDevice') }}</span>
            </dd>
            <template v-if="device">
              <dt>{{ t('devices.colLastSeen') }}</dt><dd>{{ fmtDate(device.last_seen) }}</dd>
            </template>
          </dl>
        </div>

        <!-- Current level -->
        <div class="card section level-section">
          <div class="section-title">{{ t('reservoirs.currentLevel') }}</div>
          <template v-if="latestMeasurement">
            <div class="level-value">{{ latestMeasurement.value.toFixed(1) }} <span class="level-unit">L</span></div>
            <div class="level-percent text-muted">{{ levelPercent?.toFixed(0) }}%</div>
            <div class="level-bar">
              <div class="level-fill" :style="{ width: `${levelPercent}%` }" />
            </div>
          </template>
          <p v-else class="text-muted" style="font-size:13px">{{ t('reservoirs.noDevice') }}</p>
        </div>

        <!-- Alert rules -->
        <div class="card rules-card">
          <div class="section-title">{{ t('reservoirs.rules') }}</div>

          <div class="rules-scroll">
            <div v-if="rules.length" class="rules-list">
              <div v-for="rule in rules" :key="rule.id" class="rule-row">
                <span class="rule-desc">
                  {{ conditionLabel(rule.condition_type) }} {{ rule.threshold }} L
                </span>
                <label class="toggle">
                  <input type="checkbox" :checked="rule.is_active" @change="toggleRule(rule)" />
                  <span class="toggle-track" />
                </label>
                <button class="btn btn-ghost btn-sm rule-del" @click="removeRule(rule)">✕</button>
              </div>
            </div>
            <p v-else class="text-muted" style="font-size:13px">—</p>
          </div>

          <div class="add-rule-form">
            <select v-model="newRule.condition_type" class="form-input">
              <option value="less_than">{{ t('reservoirs.ruleBelow') }}</option>
              <option value="greater_than">{{ t('reservoirs.ruleAbove') }}</option>
              <option value="equals">{{ t('reservoirs.ruleEquals') }}</option>
            </select>
            <div class="rule-bottom">
              <input
                v-model="newRule.threshold"
                type="number"
                class="form-input rule-threshold"
                :placeholder="t('reservoirs.threshold')"
              />
              <button class="btn btn-primary btn-sm" :disabled="addingRule || !newRule.threshold" @click="addRule">
                + {{ t('reservoirs.addRule') }}
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Chart -->
      <div class="card chart-card">
        <div class="section-title">{{ t('reservoirs.history') }}</div>
        <LineChart :data="chartMeasurements" class="chart-fill">
          <template #empty><span>{{ t('reservoirs.noData') }}</span></template>
        </LineChart>
      </div>
    </div>

    <!-- Edit modal -->
    <ModalDialog v-model="showEdit" :title="t('reservoirs.editTitle')">
      <div class="form-group">
        <label class="form-label">{{ t('reservoirs.name') }}</label>
        <input v-model="editForm.name" type="text" class="form-input" />
      </div>
      <div class="form-group">
        <label class="form-label">{{ t('reservoirs.capacity') }}</label>
        <input v-model="editForm.capacity" type="number" class="form-input" />
      </div>
      <div class="form-group">
        <label class="form-label">{{ t('reservoirs.location') }}</label>
        <input v-model="editForm.location" type="text" class="form-input" />
      </div>
      <div class="form-group">
        <label class="form-label">{{ t('reservoirs.description') }}</label>
        <textarea v-model="editForm.description" class="form-input" rows="2" style="resize:vertical" />
      </div>
      <template #footer>
        <button class="btn btn-outline" @click="showEdit = false">{{ t('common.cancel') }}</button>
        <button class="btn btn-primary" @click="saveEdit">{{ t('common.save') }}</button>
      </template>
    </ModalDialog>
  </div>
</template>

<style scoped>
/* ── Page layout ── */
.page-root {
  display: flex;
  flex-direction: column;
  height: 100%;
}
.back-btn { margin-bottom: 16px; flex-shrink: 0; }

.reservoir-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-height: 0;
}

/* ── Top grid ── */
.detail-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  flex-shrink: 0;
  align-items: stretch;
}
@media (max-width: 900px) {
  .detail-grid { grid-template-columns: repeat(2, 1fr); }
}
@media (max-width: 580px) {
  .detail-grid { grid-template-columns: 1fr; }
}

.section { padding: 20px; }
.section-title {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 14px;
  flex-shrink: 0;
}

.info { display: grid; grid-template-columns: max-content 1fr; gap: 8px 16px; }
.info dt { color: var(--text-muted); font-size: 12px; font-weight: 500; align-self: center; }
.info dd { font-weight: 500; }

.level-value   { font-size: 42px; font-weight: 700; line-height: 1; margin-bottom: 4px; }
.level-unit    { font-size: 20px; font-weight: 400; }
.level-percent { font-size: 14px; margin-bottom: 12px; }
.level-bar     { height: 8px; background: var(--border); border-radius: 4px; overflow: hidden; }
.level-fill    { height: 100%; background: var(--primary); border-radius: 4px; transition: width 0.4s; }

/* ── Rules card ── */
.rules-card {
  display: flex;
  flex-direction: column;
  padding: 20px;
  overflow: hidden;
}

.rules-scroll {
  overflow-y: auto;
  max-height: 120px;
  margin-bottom: 12px;
}

.rules-list { display: flex; flex-direction: column; gap: 8px; }
.rule-row   { display: flex; align-items: center; gap: 10px; padding: 8px 12px; background: var(--bg); border-radius: var(--radius-sm); }
.rule-desc  { flex: 1; font-size: 13px; font-weight: 500; }
.rule-del   { color: var(--text-muted); }
.rule-del:hover { color: var(--danger); }

.add-rule-form  { display: flex; flex-direction: column; gap: 8px; flex-shrink: 0; }
.rule-bottom    { display: flex; gap: 8px; }
.rule-threshold { flex: 1; min-width: 0; }

/* ── Chart card ── */
.chart-card {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 20px;
  min-height: 0;
}

.chart-fill {
  flex: 1;
  min-height: 0;
}

/* ── Toggle switch ── */
.toggle { position: relative; display: inline-block; width: 32px; height: 18px; flex-shrink: 0; }
.toggle input { display: none; }
.toggle-track {
  position: absolute; inset: 0;
  background: var(--border); border-radius: 9px; cursor: pointer;
  transition: background 0.2s;
}
.toggle input:checked + .toggle-track { background: var(--primary); }
.toggle-track::after {
  content: '';
  position: absolute;
  width: 14px; height: 14px;
  background: #fff; border-radius: 50%;
  top: 2px; left: 2px;
  transition: transform 0.2s;
}
.toggle input:checked + .toggle-track::after { transform: translateX(14px); }
</style>
