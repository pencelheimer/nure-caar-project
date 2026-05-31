<script setup>
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import * as deviceApi from '../../api/devices'
import * as reservoirApi from '../../api/reservoirs'
import { useToastStore }   from '../../stores/toast'
import { useConfirmStore } from '../../stores/confirm'
import Spinner     from '../../components/Spinner.vue'
import ModalDialog from '../../components/ModalDialog.vue'

const { t }   = useI18n()
const toast   = useToastStore()
const confirm = useConfirmStore()

const devices    = ref([])
const reservoirs = ref([])
const loading    = ref(true)

const showCreate = ref(false)
const form       = ref({ name: '', reservoir_id: '' })
const saving     = ref(false)

const showEdit   = ref(false)
const editTarget = ref(null)
const editForm   = ref({ name: '', reservoir_id: '' })
const savingEdit = ref(false)

const apiKeyInfo = ref(null)

onMounted(async () => {
  try {
    const [devList, resList] = await Promise.all([
      deviceApi.getDevices(),
      reservoirApi.getReservoirs(),
    ])
    devices.value    = devList
    reservoirs.value = resList
  } catch {
    toast.error(t('devices.failLoad'))
  } finally {
    loading.value = false
  }
})

function reservoirName(id) {
  return reservoirs.value.find(r => r.id === id)?.name ?? t('devices.noReservoir')
}

function openCreate() {
  form.value = { name: '', reservoir_id: '' }
  showCreate.value = true
}

async function submit() {
  if (!form.value.name) return
  saving.value = true
  try {
    const created = await deviceApi.createDevice({
      name:         form.value.name,
      reservoir_id: form.value.reservoir_id ? parseInt(form.value.reservoir_id) : null,
    })
    devices.value.push({ ...created, status: 'offline', last_seen: null })
    showCreate.value = false
    if (created.api_key) {
      apiKeyInfo.value = { deviceName: form.value.name, key: created.api_key }
    }
    toast.success(t('devices.created'))
  } catch {
    toast.error(t('devices.failCreate'))
  } finally {
    saving.value = false
  }
}

function openEdit(device) {
  editTarget.value = device
  editForm.value   = { name: device.name, reservoir_id: device.reservoir_id ?? '' }
  showEdit.value   = true
}

async function saveEdit() {
  if (!editForm.value.name) return
  savingEdit.value = true
  try {
    const updated = await deviceApi.updateDevice(editTarget.value.id, {
      name:         editForm.value.name,
      reservoir_id: editForm.value.reservoir_id !== '' ? parseInt(editForm.value.reservoir_id) : null,
    })
    Object.assign(editTarget.value, updated)
    showEdit.value = false
    toast.success(t('devices.updated'))
  } catch {
    toast.error(t('devices.failUpdate'))
  } finally {
    savingEdit.value = false
  }
}

async function rotateKey(device) {
  const ok = await confirm.confirm({
    title:        t('devices.rotateTitle'),
    message:      t('devices.rotateMessage'),
    confirmLabel: t('devices.rotateKey'),
    danger: true,
  })
  if (!ok) return
  try {
    const res = await deviceApi.rotateKey(device.id)
    apiKeyInfo.value = { deviceName: device.name, key: res.new_api_key }
    toast.success(t('devices.keyRotated'))
  } catch {
    toast.error(t('devices.failRotate'))
  }
}

async function removeDevice(device) {
  const ok = await confirm.confirm({
    title:        t('devices.deleteTitle'),
    message:      t('devices.deleteMessage', { name: device.name }),
    confirmLabel: t('common.delete'),
    danger: true,
  })
  if (!ok) return
  try {
    await deviceApi.deleteDevice(device.id)
    devices.value = devices.value.filter(d => d.id !== device.id)
    toast.success(t('devices.deleted'))
  } catch {
    toast.error(t('devices.failDelete'))
  }
}

function copyKey() {
  navigator.clipboard.writeText(apiKeyInfo.value.key).then(() => {
    toast.success(t('devices.copied'))
  })
}

function fmtDate(dt) {
  return dt ? new Date(dt).toLocaleString() : t('devices.never')
}

function statusClass(s) {
  return { online: 'badge-success', offline: 'badge-danger', maintenance: 'badge-warning' }[s] ?? ''
}
</script>

<template>
  <div>
    <div class="page-header">
      <h1 class="page-title">{{ t('devices.title') }}</h1>
      <button class="btn btn-primary btn-sm" @click="openCreate">+ {{ t('devices.add') }}</button>
    </div>

    <Spinner v-if="loading" />

    <div v-else-if="devices.length === 0" class="empty-state">
      <p class="text-muted">{{ t('devices.none') }}</p>
    </div>

    <div v-else class="card" style="padding:0;overflow:hidden">
      <table class="table">
        <thead>
          <tr>
            <th>{{ t('devices.colName') }}</th>
            <th>{{ t('devices.colStatus') }}</th>
            <th>{{ t('devices.colReservoir') }}</th>
            <th>{{ t('devices.colLastSeen') }}</th>
            <th>{{ t('devices.colActions') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="d in devices" :key="d.id">
            <td style="font-weight:500">{{ d.name }}</td>
            <td>
              <span :class="`badge ${statusClass(d.status)}`">
                {{ t(`devices.${d.status}`) }}
              </span>
            </td>
            <td class="text-muted" style="font-size:13px">{{ reservoirName(d.reservoir_id) }}</td>
            <td class="text-muted" style="font-size:13px">{{ fmtDate(d.last_seen) }}</td>
            <td>
              <div style="display:flex;gap:6px">
                <button class="btn btn-outline btn-sm" @click="openEdit(d)">{{ t('common.edit') }}</button>
                <button class="btn btn-outline btn-sm" @click="rotateKey(d)">{{ t('devices.rotateKey') }}</button>
                <button class="btn btn-danger btn-sm" @click="removeDevice(d)">{{ t('common.delete') }}</button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Create modal -->
    <ModalDialog v-model="showCreate" :title="t('devices.addTitle')">
      <div class="form-group">
        <label class="form-label">{{ t('devices.name') }}</label>
        <input v-model="form.name" type="text" class="form-input" />
      </div>
      <div class="form-group">
        <label class="form-label">{{ t('devices.reservoir') }} <span class="text-muted">({{ t('reservoirs.optional') }})</span></label>
        <select v-model="form.reservoir_id" class="form-input">
          <option value="">—</option>
          <option v-for="r in reservoirs" :key="r.id" :value="r.id">{{ r.name }}</option>
        </select>
      </div>
      <template #footer>
        <button class="btn btn-outline" @click="showCreate = false">{{ t('common.cancel') }}</button>
        <button class="btn btn-primary" :disabled="saving || !form.name" @click="submit">
          {{ saving ? t('common.loading') : t('devices.add') }}
        </button>
      </template>
    </ModalDialog>

    <!-- Edit modal -->
    <ModalDialog v-model="showEdit" :title="t('devices.editTitle')">
      <div class="form-group">
        <label class="form-label">{{ t('devices.name') }}</label>
        <input v-model="editForm.name" type="text" class="form-input" />
      </div>
      <div class="form-group">
        <label class="form-label">{{ t('devices.reservoir') }}</label>
        <select v-model="editForm.reservoir_id" class="form-input">
          <option value="">— {{ t('devices.noReservoir') }}</option>
          <option v-for="r in reservoirs" :key="r.id" :value="r.id">{{ r.name }}</option>
        </select>
      </div>
      <template #footer>
        <button class="btn btn-outline" @click="showEdit = false">{{ t('common.cancel') }}</button>
        <button class="btn btn-primary" :disabled="savingEdit || !editForm.name" @click="saveEdit">
          {{ savingEdit ? t('common.loading') : t('common.save') }}
        </button>
      </template>
    </ModalDialog>

    <!-- API key display modal -->
    <ModalDialog v-if="apiKeyInfo" :model-value="true" :title="t('devices.newKeyTitle')" @update:model-value="apiKeyInfo = null">
      <p style="font-size:13px;color:var(--text-muted);margin-bottom:12px">{{ t('devices.apiKeyNote') }}</p>
      <div class="key-box">
        <code class="key-value">{{ apiKeyInfo.key }}</code>
        <button class="btn btn-outline btn-sm" @click="copyKey">{{ t('devices.copy') }}</button>
      </div>
      <template #footer>
        <button class="btn btn-primary" @click="apiKeyInfo = null">OK</button>
      </template>
    </ModalDialog>
  </div>
</template>

<style scoped>
.empty-state { text-align: center; padding: 60px; }

.key-box {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px;
  background: var(--surface-alt);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border);
}
.key-value {
  flex: 1;
  font-size: 12px;
  word-break: break-all;
  color: var(--text);
  font-family: monospace;
}
</style>
