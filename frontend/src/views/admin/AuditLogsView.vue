<script setup>
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import * as adminApi from '../../api/admin'
import { useToastStore } from '../../stores/toast'
import Spinner from '../../components/Spinner.vue'

const { t } = useI18n()
const toast = useToastStore()

const logs     = ref([])
const loading  = ref(false)
const hasMore  = ref(false)
const expanded = ref(new Set())

const tableFilter     = ref('')
const operationFilter = ref('')
let   offset = 0
const PAGE = 50

const TABLES     = ['user', 'reservoir', 'device', 'alert_rule', 'alert']
const OPERATIONS = ['INSERT', 'UPDATE', 'DELETE']

onMounted(() => load(true))

async function load(reset = false) {
  if (reset) {
    offset = 0
    logs.value = []
    expanded.value.clear()
  }
  loading.value = true
  try {
    const params = { limit: PAGE + 1, offset }
    if (tableFilter.value)     params.table_name = tableFilter.value
    if (operationFilter.value) params.operation   = operationFilter.value

    const batch = await adminApi.getAuditLogs(params)
    hasMore.value = batch.length > PAGE
    const page = hasMore.value ? batch.slice(0, PAGE) : batch
    logs.value = reset ? page : [...logs.value, ...page]
    offset += page.length
  } catch {
    toast.error(t('auditLogs.failLoad'))
  } finally {
    loading.value = false
  }
}

function toggle(id) {
  if (expanded.value.has(id)) expanded.value.delete(id)
  else expanded.value.add(id)
  expanded.value = new Set(expanded.value)
}

function hasDetails(log) {
  return log.old_values != null || log.new_values != null
}

function fmt(dt) {
  return new Date(dt).toLocaleString()
}

function fmtJson(val) {
  if (val == null) return null
  try { return JSON.stringify(typeof val === 'string' ? JSON.parse(val) : val, null, 2) }
  catch { return String(val) }
}

function opClass(op) {
  return { INSERT: 'badge-success', UPDATE: 'badge-warning', DELETE: 'badge-danger' }[op] ?? ''
}
</script>

<template>
  <div>
    <div class="page-header">
      <h1 class="page-title">{{ t('auditLogs.title') }}</h1>
    </div>

    <div class="toolbar">
      <select v-model="tableFilter" class="form-input filter" @change="load(true)">
        <option value="">{{ t('auditLogs.allTables') }}</option>
        <option v-for="tbl in TABLES" :key="tbl" :value="tbl">{{ tbl }}</option>
      </select>
      <select v-model="operationFilter" class="form-input filter" @change="load(true)">
        <option value="">{{ t('auditLogs.allOperations') }}</option>
        <option v-for="op in OPERATIONS" :key="op" :value="op">{{ op }}</option>
      </select>
      <button class="btn btn-outline btn-sm" :disabled="loading" @click="load(true)">
        {{ t('common.refresh') }}
      </button>
    </div>

    <div class="card">
      <div class="table-wrap">
        <table>
          <thead>
            <tr>
              <th style="width:32px"></th>
              <th>{{ t('auditLogs.colTime') }}</th>
              <th>{{ t('auditLogs.colTable') }}</th>
              <th>{{ t('auditLogs.colOperation') }}</th>
              <th>{{ t('auditLogs.colRecordId') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="loading && logs.length === 0">
              <td colspan="5"><Spinner /></td>
            </tr>
            <tr v-else-if="!loading && logs.length === 0">
              <td colspan="5" class="empty-row">{{ t('auditLogs.notFound') }}</td>
            </tr>

            <template v-for="log in logs" :key="log.id">
              <tr
                :class="['log-row', { expandable: hasDetails(log), expanded: expanded.has(log.id) }]"
                @click="hasDetails(log) && toggle(log.id)"
              >
                <td class="expand-cell">
                  <span v-if="hasDetails(log)" class="expand-icon">
                    {{ expanded.has(log.id) ? '▾' : '▸' }}
                  </span>
                </td>
                <td class="text-muted mono">{{ fmt(log.changed_at) }}</td>
                <td><code class="code-tag">{{ log.table_name }}</code></td>
                <td><span :class="`badge ${opClass(log.operation)}`">{{ log.operation }}</span></td>
                <td>{{ log.record_id ?? '—' }}</td>
              </tr>

              <tr v-if="expanded.has(log.id)" class="detail-row">
                <td colspan="5">
                  <div class="detail-grid">
                    <div v-if="log.old_values != null" class="detail-block">
                      <div class="detail-label">{{ t('auditLogs.before') }}</div>
                      <pre class="detail-json">{{ fmtJson(log.old_values) }}</pre>
                    </div>
                    <div v-if="log.new_values != null" class="detail-block">
                      <div class="detail-label">{{ t('auditLogs.after') }}</div>
                      <pre class="detail-json">{{ fmtJson(log.new_values) }}</pre>
                    </div>
                  </div>
                </td>
              </tr>
            </template>
          </tbody>
        </table>
      </div>
      <div v-if="hasMore" class="pagination">
        <button class="btn btn-outline" :disabled="loading" @click="load(false)">
          {{ loading ? t('common.loading') : t('common.loadMore') }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.toolbar { display: flex; gap: 10px; margin-bottom: 16px; align-items: center; }
.filter  { width: 160px; }
.mono    { font-size: 12px; }
.code-tag {
  background: var(--bg);
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 12px;
  font-family: monospace;
}
.log-row.expandable { cursor: pointer; }
.log-row.expanded td { background: var(--surface-alt); }
.expand-cell { width: 32px; text-align: center; }
.expand-icon { color: var(--text-muted); font-size: 12px; user-select: none; }
.detail-row td { padding: 0; background: var(--surface-alt); border-bottom: 1px solid var(--border); }
.detail-row:hover td { background: var(--surface-alt); }
.detail-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 1px;
  background: var(--border);
}
.detail-block { background: var(--surface-alt); padding: 12px 16px; }
.detail-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 8px;
}
.detail-json {
  font-family: monospace;
  font-size: 12px;
  color: var(--text);
  white-space: pre-wrap;
  word-break: break-all;
  margin: 0;
  line-height: 1.6;
}
</style>
