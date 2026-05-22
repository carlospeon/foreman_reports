import { Chart, Title, Tooltip, Legend, Colors } from 'chart.js'

export function registerChartPlugins() {
  Chart.register(Title, Tooltip, Legend, Colors);
  Chart.defaults.font.size = 13;
  Chart.defaults.font.family = "'abc_monument_grotesk_Lt', 'Inter', 'Helvetica Neue', sans-serif";
}

export function getThemeColors() {
  const isDark = document.documentElement.dataset.theme === 'dark';
  return {
    text: isDark ? '#FFFFFF' : '#000000',
    textSecondary: isDark ? '#6B7280' : '#374151',
    grid: isDark ? '#374151' : '#e5e7eb',
    surface: isDark ? '#121212' : '#FFFFFF',
  };
}

export const CHART_COLORS = [
  '#36a2eb', '#ff6384', '#4bc0c0', '#ff9f40', '#9966ff',
  '#ffcd56', '#c9cbcf', '#e1b496', '#82d2f5', '#9be1af',
  '#e1d2af', '#4b646e', '#dc3282',
];

export function doughnutOptions(title) {
  const colors = getThemeColors();
  return {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      title: {
        display: true,
        text: title,
        color: colors.text,
      },
      legend: {
        position: 'bottom',
        align: 'start',
        labels: {
          color: colors.textSecondary,
          generateLabels: (chart) => {
            const datasets = chart.data.datasets;
            return datasets[0].data.map((data, i) => ({
              text: `${chart.data.labels[i]}: ${data}`,
              fillStyle: datasets[0].backgroundColor[i],
              fontColor: colors.textSecondary,
            }));
          },
        },
      },
    },
  };
}

export function barOptions(title, yLabel = 'Hosts', opts = {}) {
  const colors = getThemeColors();
  return {
    responsive: true,
    maintainAspectRatio: false,
    scales: {
      x: {
        ticks: { color: colors.textSecondary },
        grid: { color: colors.grid },
        ...(opts.x || {}),
      },
      y: {
        title: { display: true, text: yLabel, color: colors.text },
        stacked: true,
        ticks: { color: colors.textSecondary },
        grid: { color: colors.grid },
        ...(opts.y || {}),
      },
    },
    plugins: {
      title: {
        display: true,
        text: title,
        color: colors.text,
      },
      legend: {
        position: 'bottom',
        labels: { color: colors.textSecondary },
        ...(opts.legend || {}),
      },
    },
    ...(opts.extra || {}),
  };
}

export function lineOptions(title, xLabel = 'Week', yLabel = 'Hosts', opts = {}) {
  const colors = getThemeColors();
  return {
    responsive: true,
    maintainAspectRatio: false,
    scales: {
      x: {
        title: { display: true, text: xLabel, color: colors.text },
        ticks: { color: colors.textSecondary },
        grid: { color: colors.grid },
      },
      y: {
        title: { display: true, text: yLabel, color: colors.text },
        ticks: { color: colors.textSecondary },
        grid: { color: colors.grid },
        ...(opts.y || {}),
      },
    },
    plugins: {
      title: {
        display: true,
        text: title,
        color: colors.text,
      },
      legend: {
        position: 'bottom',
        labels: { color: colors.textSecondary },
      },
    },
  };
}
