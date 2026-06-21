export function getStatusColor(percentage: number): string {
  if (percentage >= 90) return '#EF4444';
  if (percentage >= 70) return '#F59E0B';
  return '#34c759';
}

export function getStatusClass(percentage: number): string {
  if (percentage >= 90) return 'text-status-danger';
  if (percentage >= 70) return 'text-status-warning';
  return 'text-status-ok';
}

export function getStatusDotClass(percentage: number): string {
  if (percentage >= 90) return 'bg-status-danger';
  if (percentage >= 70) return 'bg-status-warning';
  return 'bg-status-ok';
}
