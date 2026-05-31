<script setup>
import { computed } from 'vue'
import { Line } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Tooltip,
  Filler,
} from 'chart.js'
import { useThemeStore } from '../stores/theme'

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Tooltip, Filler)

const props = defineProps({
  data: { type: Array, default: () => [] },
})

const theme = useThemeStore()

const palette = computed(() => {
  const dark = theme.effectiveTheme === 'dark'
  return {
    line:    dark ? '#60a5fa' : '#2563eb',
    fill:    dark ? 'rgba(96,165,250,0.12)' : 'rgba(37,99,235,0.08)',
    grid:    dark ? '#2d3f55' : '#e2e8f0',
    muted:   dark ? '#94a3b8' : '#64748b',
    tooltip: dark ? '#1e293b' : '#ffffff',
    ttBorder:dark ? '#2d3f55' : '#e2e8f0',
    ttText:  dark ? '#f1f5f9' : '#0f172a',
  }
})

function fmtLabel(dt) {
  return new Date(dt).toLocaleString(undefined, {
    month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit',
  })
}

const chartData = computed(() => ({
  labels: props.data.map(d => fmtLabel(d.time)),
  datasets: [{
    data:            props.data.map(d => d.value),
    borderColor:     palette.value.line,
    backgroundColor: palette.value.fill,
    borderWidth:     2,
    pointRadius:     0,
    pointHoverRadius: 4,
    fill:            true,
    tension:         0.35,
  }],
}))

const options = computed(() => ({
  responsive:          true,
  maintainAspectRatio: false,
  animation:           false,
  plugins: {
    legend: { display: false },
    tooltip: {
      backgroundColor: palette.value.tooltip,
      borderColor:     palette.value.ttBorder,
      borderWidth:     1,
      titleColor:      palette.value.muted,
      bodyColor:       palette.value.ttText,
      bodyFont:        { size: 13, weight: '600' },
      padding:         10,
      callbacks: {
        label: ctx => `${ctx.parsed.y.toFixed(1)} L`,
      },
    },
  },
  scales: {
    x: {
      grid:  { display: false },
      border: { display: false },
      ticks: {
        maxTicksLimit: 6,
        color:     palette.value.muted,
        font:      { size: 11 },
        maxRotation: 0,
      },
    },
    y: {
      grid:  { color: palette.value.grid, lineWidth: 1 },
      border: { display: false, dash: [4, 3] },
      ticks: {
        color:     palette.value.muted,
        font:      { size: 11 },
        callback:  v => `${v} L`,
      },
    },
  },
  interaction: {
    mode:      'index',
    intersect: false,
  },
}))
</script>

<template>
  <div class="chart-wrap">
    <Line v-if="data.length >= 2" :data="chartData" :options="options" />
    <div v-else class="no-data">
      <slot name="empty" />
    </div>
  </div>
</template>

<style scoped>
.chart-wrap { width: 100%; height: 100%; min-height: 160px; }
.no-data    { display: flex; justify-content: center; align-items: center; height: 100%; min-height: 160px; color: var(--text-muted); font-size: 13px; }
</style>
