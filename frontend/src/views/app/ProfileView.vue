<script setup>
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '../../stores/auth'
import { useToastStore } from '../../stores/toast'
import client from '../../api/client'
import EyeIcon from '../../components/EyeIcon.vue'

const { t }   = useI18n()
const auth    = useAuthStore()
const toast   = useToastStore()

const profile  = ref(null)
const loading  = ref(true)

const editForm = ref({ first_name: '', last_name: '' })
const saving   = ref(false)

const pwForm      = ref({ current_password: '', new_password: '' })
const savingPw    = ref(false)
const showCurrent = ref(false)
const showNew     = ref(false)

onMounted(async () => {
  try {
    profile.value = await client.get('/auth/me')
    editForm.value = {
      first_name: profile.value.first_name ?? '',
      last_name:  profile.value.last_name  ?? '',
    }
  } catch {
    // auth store already has user info as fallback
    if (auth.user) {
      profile.value = auth.user
      editForm.value = {
        first_name: auth.user.first_name ?? '',
        last_name:  auth.user.last_name  ?? '',
      }
    }
  } finally {
    loading.value = false
  }
})

async function saveProfile() {
  saving.value = true
  try {
    const updated = await client.put('/auth/me', {
      first_name: editForm.value.first_name || null,
      last_name:  editForm.value.last_name  || null,
    })
    Object.assign(profile.value, updated)
    toast.success(t('profile.saved'))
  } catch {
    toast.error(t('profile.failSave'))
  } finally {
    saving.value = false
  }
}

async function changePassword() {
  if (!pwForm.value.current_password || !pwForm.value.new_password) return
  savingPw.value = true
  try {
    await client.post('/auth/change-password', {
      current_password: pwForm.value.current_password,
      new_password:     pwForm.value.new_password,
    })
    pwForm.value = { current_password: '', new_password: '' }
    toast.success(t('profile.passwordChanged'))
  } catch {
    toast.error(t('profile.failPassword'))
  } finally {
    savingPw.value = false
  }
}

function fmtDate(dt) {
  return dt ? new Date(dt).toLocaleDateString() : '—'
}
</script>

<template>
  <div>
    <h1 class="page-title" style="margin-bottom:20px">{{ t('profile.title') }}</h1>

    <div v-if="loading" style="padding:40px;text-align:center" class="text-muted">{{ t('common.loading') }}</div>

    <template v-else-if="profile">
      <div class="profile-grid">
        <!-- Profile info + edit -->
        <div class="card section">
          <div class="section-title">{{ t('profile.editTitle') }}</div>
          <dl class="info" style="margin-bottom:20px">
            <dt>{{ t('profile.email') }}</dt>
            <dd>{{ profile.email }}</dd>
            <dt>{{ t('profile.memberSince') }}</dt>
            <dd>{{ fmtDate(profile.created_at) }}</dd>
          </dl>

          <div class="form-group">
            <label class="form-label">{{ t('profile.firstName') }}</label>
            <input v-model="editForm.first_name" type="text" class="form-input" />
          </div>
          <div class="form-group">
            <label class="form-label">{{ t('profile.lastName') }}</label>
            <input v-model="editForm.last_name" type="text" class="form-input" />
          </div>
          <div class="form-footer">
            <button class="btn btn-primary" :disabled="saving" @click="saveProfile">
              {{ saving ? t('common.loading') : t('common.save') }}
            </button>
          </div>
        </div>

        <!-- Change password -->
        <div class="card section">
          <div class="section-title">{{ t('profile.changePassword') }}</div>
          <div class="form-group">
            <label class="form-label">{{ t('profile.currentPassword') }}</label>
            <div class="input-wrap">
              <input v-model="pwForm.current_password" :type="showCurrent ? 'text' : 'password'" class="form-input" autocomplete="current-password" />
              <button type="button" class="eye-btn" @click="showCurrent = !showCurrent"><EyeIcon :open="!showCurrent" /></button>
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">{{ t('profile.newPassword') }}</label>
            <div class="input-wrap">
              <input v-model="pwForm.new_password" :type="showNew ? 'text' : 'password'" class="form-input" autocomplete="new-password" />
              <button type="button" class="eye-btn" @click="showNew = !showNew"><EyeIcon :open="!showNew" /></button>
            </div>
          </div>
          <div class="form-footer">
            <button
              class="btn btn-primary"
              :disabled="savingPw || !pwForm.current_password || !pwForm.new_password"
              @click="changePassword"
            >
              {{ savingPw ? t('common.loading') : t('profile.changePassword') }}
            </button>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.profile-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 16px;
}
.section { padding: 20px; }
.section-title {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 16px;
}
.info { display: grid; grid-template-columns: max-content 1fr; gap: 8px 16px; }
.info dt { color: var(--text-muted); font-size: 12px; font-weight: 500; align-self: center; }
.info dd { font-weight: 500; }

.form-footer { margin-top: 8px; }

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
