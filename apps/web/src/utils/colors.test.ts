import { describe, it, expect } from 'vitest';
import {
  getStatusColor,
  getStatusClass,
  getStatusDotClass,
} from './colors';

describe('getStatusColor', () => {
  it('returns green for usage below the warning threshold', () => {
    expect(getStatusColor(0)).toBe('#34c759');
    expect(getStatusColor(69)).toBe('#34c759');
    expect(getStatusColor(69.99)).toBe('#34c759');
  });

  it('returns amber at and above 70 up to 89', () => {
    expect(getStatusColor(70)).toBe('#F59E0B');
    expect(getStatusColor(89)).toBe('#F59E0B');
    expect(getStatusColor(85)).toBe('#F59E0B');
  });

  it('returns red at and above 90', () => {
    expect(getStatusColor(90)).toBe('#EF4444');
    expect(getStatusColor(100)).toBe('#EF4444');
  });
});

describe('getStatusClass', () => {
  it('maps thresholds to text status classes', () => {
    expect(getStatusClass(10)).toBe('text-status-ok');
    expect(getStatusClass(70)).toBe('text-status-warning');
    expect(getStatusClass(90)).toBe('text-status-danger');
  });
});

describe('getStatusDotClass', () => {
  it('maps thresholds to background status classes', () => {
    expect(getStatusDotClass(10)).toBe('bg-status-ok');
    expect(getStatusDotClass(70)).toBe('bg-status-warning');
    expect(getStatusDotClass(90)).toBe('bg-status-danger');
  });
});
