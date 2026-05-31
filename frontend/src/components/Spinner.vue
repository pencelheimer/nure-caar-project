<template>
  <div class="spinner-wrap">
    <div class="spinner"></div>
  </div>
</template>

<style scoped>
.spinner-wrap { display: flex; justify-content: center; padding: 32px; }
.spinner {
  width: 28px;
  height: 28px;
  border: 3px solid var(--border);
  border-top-color: var(--primary);
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}
@keyframes spin { to { transform: rotate(360deg); } }
</style>
