<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import * as adminApi from '../../api/admin'
import { useToastStore }   from '../../stores/toast'
import { useConfirmStore } from '../../stores/confirm'
import Spinner from '../../components/Spinner.vue'

const { t } = useI18n()
const router  = useRouter()
const toast   = useToastStore()
const confirm = useConfirmStore()

const users      = ref([])
const loading    = ref(true)
const search     = ref('')
const roleFilter = ref('')

onMounted(loadUsers)

async function loadUsers() {
  loading.value = true
  try {
    users.value = await adminApi.getUsers()
  } catch {
    toast.error(t('users.failLoad'))
  } finally {
    loading.value = false
  }
}

const filtered = computed(() => {
  const q = search.value.toLowerCase()
  return users.value.filter(u => {
    const matchSearch = !q
      || u.email.toLowerCase().includes(q)
      || (u.first_name ?? '').toLowerCase().includes(q)
      || (u.last_name  ?? '').toLowerCase().includes(q)
    const matchRole = !roleFilter.value || u.role === roleFilter.value
    return matchSearch && matchRole
  })
})

async function setRole(user, role) {
  try {
    await adminApi.setUserRole(user.id, role)
    user.role = role
    toast.success(t('users.roleUpdated'))
  } catch {
    toast.error(t('users.failRole'))
  }
}

async function toggleBan(user) {
  if (user.is_banned) {
    const ok = await confirm.confirm({
      title: t('users.unbanTitle'),
      message: t('users.unbanMessage', { email: user.email }),
      confirmLabel: t('users.unban'),
    })
    if (!ok) return
    try {
      await adminApi.banUser(user.id, { is_banned: false })
      user.is_banned = false
      toast.success(t('users.unbanned'))
    } catch {
      toast.error(t('users.failUnban'))
    }
  } else {
    const ok = await confirm.confirm({
      title: t('users.banTitle'),
      message: t('users.banMessage', { email: user.email }),
      confirmLabel: t('users.ban'),
      danger: true,
    })
    if (!ok) return
    try {
      await adminApi.banUser(user.id, { is_banned: true })
      user.is_banned = true
      toast.success(t('users.banned'))
    } catch {
      toast.error(t('users.failBan'))
    }
  }
}

async function removeUser(user) {
  const ok = await confirm.confirm({
    title: t('users.deleteTitle'),
    message: t('users.deleteMessage', { email: user.email }),
    confirmLabel: t('common.delete'),
    danger: true,
  })
  if (!ok) return
  try {
    await adminApi.deleteUser(user.id)
    users.value = users.value.filter(u => u.id !== user.id)
    toast.success(t('users.deleted'))
  } catch {
    toast.error(t('users.failDelete'))
  }
}

function displayName(u) {
  return [u.first_name, u.last_name].filter(Boolean).join(' ') || '—'
}
</script>

<template>
  <div>
    <div class="page-header">
      <h1 class="page-title">{{ t('users.title') }}</h1>
    </div>

    <div class="toolbar">
      <input
        v-model="search"
        type="search"
        class="form-input search-input"
        :placeholder="t('users.searchPlaceholder')"
      />
      <select v-model="roleFilter" class="form-input role-filter">
        <option value="">{{ t('users.allRoles') }}</option>
        <option value="admin">admin</option>
        <option value="user">user</option>
        <option value="viewer">viewer</option>
      </select>
    </div>

    <div class="card">
      <Spinner v-if="loading" />
      <div v-else class="table-wrap">
        <table>
          <thead>
            <tr>
              <th>{{ t('users.colUser') }}</th>
              <th>{{ t('users.colRole') }}</th>
              <th>{{ t('users.colReservoirs') }}</th>
              <th>{{ t('users.colDevices') }}</th>
              <th>{{ t('users.colStatus') }}</th>
              <th>{{ t('users.colActions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="filtered.length === 0">
              <td colspan="6" class="empty-row">{{ t('users.notFound') }}</td>
            </tr>
            <tr
              v-for="user in filtered"
              :key="user.id"
              class="row-clickable"
              @click="router.push(`/admin/users/${user.id}`)"
            >
              <td>
                <div class="user-cell">
                  <span class="user-name">{{ displayName(user) }}</span>
                  <span class="text-muted" style="font-size:12px">{{ user.email }}</span>
                </div>
              </td>
              <td @click.stop>
                <select
                  :value="user.role"
                  class="form-input role-select"
                  @change="setRole(user, $event.target.value)"
                >
                  <option value="admin">admin</option>
                  <option value="user">user</option>
                  <option value="viewer">viewer</option>
                </select>
              </td>
              <td>{{ user.reservoirs_count }}</td>
              <td>{{ user.devices_count }}</td>
              <td>
                <span :class="user.is_banned ? 'badge badge-danger' : 'badge badge-success'">
                  {{ user.is_banned ? t('users.statusBanned') : t('users.statusActive') }}
                </span>
              </td>
              <td @click.stop>
                <div class="actions">
                  <button class="btn btn-outline btn-sm" @click="toggleBan(user)">
                    {{ user.is_banned ? t('users.unban') : t('users.ban') }}
                  </button>
                  <button class="btn btn-ghost btn-sm btn-delete" @click="removeUser(user)">
                    {{ t('common.delete') }}
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<style scoped>
.toolbar { display: flex; gap: 10px; margin-bottom: 16px; }
.search-input { flex: 1; max-width: 320px; }
.role-filter  { width: 140px; }
.row-clickable { cursor: pointer; }
.user-cell  { display: flex; flex-direction: column; }
.user-name  { font-weight: 500; }
.role-select { width: 90px; padding: 4px 6px; font-size: 12px; }
.actions    { display: flex; gap: 6px; }
.btn-delete { color: var(--danger) !important; }
.btn-delete:hover { background: var(--primary-light) !important; }
</style>
