<script setup>
import { useI18n } from 'vue-i18n'
import { useConfirmStore } from '../stores/confirm'
const { t } = useI18n()
const confirm = useConfirmStore()
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog">
      <div v-if="confirm.dialog" class="overlay" @click.self="confirm.answer(false)">
        <div class="dialog card">
          <h2 class="dialog-title">{{ confirm.dialog.title }}</h2>
          <p class="dialog-body text-muted">{{ confirm.dialog.message }}</p>
          <div class="dialog-actions">
            <button class="btn btn-outline" @click="confirm.answer(false)">{{ t('confirm.cancel') }}</button>
            <button
              class="btn"
              :class="confirm.dialog.danger ? 'btn-danger' : 'btn-primary'"
              @click="confirm.answer(true)"
            >
              {{ confirm.dialog.confirmLabel }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9998;
}
.dialog {
  width: 100%;
  max-width: 420px;
  padding: 28px;
}
.dialog-title   { font-size: 17px; font-weight: 700; margin-bottom: 10px; }
.dialog-body    { font-size: 14px; line-height: 1.6; margin-bottom: 24px; }
.dialog-actions { display: flex; justify-content: flex-end; gap: 10px; }

.dialog-enter-from, .dialog-leave-to   { opacity: 0; }
.dialog-enter-active, .dialog-leave-active { transition: opacity 0.2s; }
</style>
