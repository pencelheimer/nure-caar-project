<script setup>
import { RouterView } from 'vue-router'
import AppToast from './components/AppToast.vue'
import ConfirmDialog from './components/ConfirmDialog.vue'
import { useThemeStore } from './stores/theme'

useThemeStore()
</script>

<template>
  <RouterView />
  <AppToast />
  <ConfirmDialog />
</template>
