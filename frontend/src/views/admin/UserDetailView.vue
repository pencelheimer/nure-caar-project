<script setup>
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import * as adminApi from '../../api/admin'
import { useToastStore }   from '../../stores/toast'
import { useConfirmStore } from '../../stores/confirm'
import Spinner from '../../components/Spinner.vue'

const { t } = useI18n()
const route   = useRoute()
const router  = useRouter()
const toast   = useToastStore()
const confirm = useConfirmStore()

const user        = ref(null)
const loading     = ref(true)
const showBanForm = ref(false)
const banReason   = ref('')

onMounted(async () => {
  try {
    user.value = await adminApi.getUser(route.params.id)
  } catch {
    toast.error(t('userDetail.notFound'))
    router.push('/admin/users')
  } finally {
    loading.value = false
  }
})

async function setRole(role) {
  try {
    await adminApi.setUserRole(user.value.id, role)
    user.value.role = role
    toast.success(t('userDetail.roleUpdated'))
  } catch {
    toast.error(t('userDetail.failRole'))
  }
}

async function ban() {
  const ok = await confirm.confirm({
    title: t('userDetail.banTitle'),
    message: t('userDetail.banMessage', { email: user.value.email }),
    confirmLabel: t('common.confirmBan'),
    danger: true,
  })
  if (!ok) return
  try {
    await adminApi.banUser(user.value.id, { is_banned: true, ban_reason: banReason.value || null })
    user.value.is_banned  = true
    user.value.ban_reason = banReason.value || null
    showBanForm.value = false
    banReason.value   = ''
    toast.success(t('userDetail.banned'))
  } catch {
    toast.error(t('userDetail.failBan'))
  }
}

async function unban() {
  const ok = await confirm.confirm({
    title: t('userDetail.unbanTitle'),
    message: t('userDetail.unbanMessage', { email: user.value.email }),
    confirmLabel: t('users.unban'),
  })
  if (!ok) return
  try {
    await adminApi.banUser(user.value.id, { is_banned: false })
    user.value.is_banned  = false
    user.value.ban_reason = null
    toast.success(t('userDetail.unbanned'))
  } catch {
    toast.error(t('userDetail.failUnban'))
  }
}

async function removeUser() {
  const ok = await confirm.confirm({
    title: t('userDetail.deleteTitle'),
    message: t('userDetail.deleteMessage', { email: user.value.email }),
    confirmLabel: t('common.delete'),
    danger: true,
  })
  if (!ok) return
  try {
    await adminApi.deleteUser(user.value.id)
    toast.success(t('userDetail.deleted'))
    router.push('/admin/users')
  } catch {
    toast.error(t('userDetail.failDelete'))
  }
}

function fmt(dt) {
  return dt ? new Date(dt).toLocaleString() : '—'
}
</script>

<template>
  <div>
    <button class="btn btn-ghost btn-sm" style="margin-bottom:16px" @click="router.back()">
      ← {{ t('common.back') }}
    </button>

    <Spinner v-if="loading" />

    <template v-else-if="user">
      <div class="page-header">
        <h1 class="page-title">
          {{ [user.first_name, user.last_name].filter(Boolean).join(' ') || user.email }}
        </h1>
        <button class="btn btn-danger btn-sm" @click="removeUser">{{ t('userDetail.deleteAccount') }}</button>
      </div>

      <div class="detail-grid">
        <div class="card section">
          <h2 class="section-title">{{ t('userDetail.sectionProfile') }}</h2>
          <dl class="info">
            <dt>{{ t('userDetail.email') }}</dt>      <dd>{{ user.email }}</dd>
            <dt>{{ t('userDetail.firstName') }}</dt>  <dd>{{ user.first_name || '—' }}</dd>
            <dt>{{ t('userDetail.lastName') }}</dt>   <dd>{{ user.last_name  || '—' }}</dd>
            <dt>{{ t('userDetail.registered') }}</dt> <dd>{{ fmt(user.created_at) }}</dd>
          </dl>
        </div>

        <div class="card section">
          <h2 class="section-title">{{ t('userDetail.sectionStats') }}</h2>
          <dl class="info">
            <dt>{{ t('userDetail.reservoirs') }}</dt> <dd>{{ user.reservoirs_count }}</dd>
            <dt>{{ t('userDetail.devices') }}</dt>    <dd>{{ user.devices_count }}</dd>
          </dl>
        </div>

        <div class="card section">
          <h2 class="section-title">{{ t('userDetail.sectionRole') }}</h2>
          <div class="role-row">
            <span :class="`badge badge-${user.role}`">{{ user.role }}</span>
            <select
              class="form-input role-select"
              :value="user.role"
              @change="setRole($event.target.value)"
            >
              <option value="admin">admin</option>
              <option value="user">user</option>
              <option value="viewer">viewer</option>
            </select>
          </div>
        </div>

        <div class="card section">
          <h2 class="section-title">{{ t('userDetail.sectionAccess') }}</h2>
          <div v-if="user.is_banned" class="access-col">
            <span class="badge badge-danger">{{ t('users.statusBanned') }}</span>
            <p v-if="user.ban_reason" class="ban-reason text-muted">{{ user.ban_reason }}</p>
            <button class="btn btn-outline btn-sm" @click="unban">{{ t('users.unban') }}</button>
          </div>
          <div v-else class="access-col">
            <span class="badge badge-success">{{ t('users.statusActive') }}</span>
            <button v-if="!showBanForm" class="btn btn-outline btn-sm btn-delete" @click="showBanForm = true">
              {{ t('userDetail.banUser') }}
            </button>
            <template v-else>
              <input v-model="banReason" class="form-input" :placeholder="t('common.banReason')" />
              <div class="ban-actions">
                <button class="btn btn-danger btn-sm" @click="ban">{{ t('common.confirmBan') }}</button>
                <button class="btn btn-ghost btn-sm" @click="showBanForm = false; banReason = ''">
                  {{ t('common.cancel') }}
                </button>
              </div>
            </template>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.detail-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 16px;
}
.section { padding: 20px; }
.section-title {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 14px;
}
.info {
  display: grid;
  grid-template-columns: max-content 1fr;
  gap: 8px 16px;
}
.info dt { color: var(--text-muted); font-size: 12px; font-weight: 500; align-self: center; }
.info dd { font-weight: 500; }
.role-row { display: flex; align-items: center; gap: 10px; }
.role-select { width: 110px; }
.access-col { display: flex; flex-direction: column; gap: 10px; }
.ban-reason { font-size: 13px; }
.ban-actions { display: flex; gap: 8px; }
.btn-delete { color: var(--danger) !important; }
</style>
