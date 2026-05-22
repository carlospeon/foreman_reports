
export function capitalize(str) {
  return str.charAt(0).toUpperCase() + str.slice(1);
}
export function factToFriendlyName(str) {
  const name = str.charAt(0).toUpperCase() + str.slice(1);
  return name.replaceAll('_', ' ');
}

export function formatNumber(n) {
  return Number.parseFloat(n).toFixed(2);
}

export function numberWithCommas(x) {
  return x.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}

export function removeDecimalsFromString(str) {
  return str.split('.')[0];
}

export function formatDate(d) {
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: 'short',
    timeStyle: 'medium'
  }).format(new Date(d));
}
