<script setup>
import { useToastStore } from '../stores/toast'
const toast = useToastStore()
</script>

<template>
  <Teleport to="body">
    <div class="toast-container">
      <TransitionGroup name="toast">
        <div
          v-for="t in toast.toasts"
          :key="t.id"
          class="toast"
          :class="`toast-${t.type}`"
          @click="toast.remove(t.id)"
        >
          {{ t.message }}
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<style scoped>
.toast-container {
  position: fixed;
  bottom: 24px;
  right: 24px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  z-index: 9999;
}
.toast {
  padding: 12px 18px;
  border-radius: var(--radius);
  font-size: 13px;
  font-weight: 500;
  color: #fff;
  cursor: pointer;
  max-width: 360px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}
.toast-success { background: var(--success); }
.toast-error   { background: var(--danger); }
.toast-info    { background: var(--primary); }

.toast-enter-from { opacity: 0; transform: translateY(8px); }
.toast-leave-to   { opacity: 0; transform: translateX(100%); }
.toast-enter-active,
.toast-leave-active { transition: all 0.25s ease; }
</style>
