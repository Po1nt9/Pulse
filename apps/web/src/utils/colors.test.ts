import { describe, it, expect } from 'vitest';
import { getStatusColor, getStatusClass, getStatusDotClass } from './colors';

// These thresholds drive the entire UI's status signalling (status dots, text
// colour, balance warnings). Locking the exact `>=` boundaries (70 and 90) is
// the highest-value regression net here: flipping `>=` to `>` would silently
// mis-classify providers at the threshold.

describe('getStatusColor', () => {
  it('returns ok green below the warning threshold', () => {
    expect(getStatusColor(0)).toBe('#34c759');
    expect(getStatusColor(69.99)).toBe('#34c759');
  });

  it('switches to warning amber exactly at 70', () => {
    expect(getStatusColor(70)).toBe('#F59E0B');
    expect(getStatusColor(89.99)).toBe('#F59E0B');
  });

  it('switches to danger red exactly at 90', () => {
    expect(getStatusColor(90)).toBe('#EF4444');
    expect(getStatusColor(100)).toBe('#EF4444');
  });
});

describe('getStatusClass', () => {
  it('returns the ok class below 70', () => {
    expect(getStatusClass(0)).toBe('text-status-ok');
    expect(getStatusClass(69.99)).toBe('text-status-ok');
  });

  it('returns the warning class at and above 70 (below 90)', () => {
    expect(getStatusClass(70)).toBe('text-status-warning');
    expect(getStatusClass(89.99)).toBe('text-status-warning');
  });

  it('returns the danger class at and above 90', () => {
    expect(getStatusClass(90)).toBe('text-status-danger');
    expect(getStatusClass(100)).toBe('text-status-danger');
  });
});

describe('getStatusDotClass', () => {
  it('returns the ok dot class below 70', () => {
    expect(getStatusDotClass(0)).toBe('bg-status-ok');
    expect(getStatusDotClass(69.99)).toBe('bg-status-ok');
  });

  it('returns the warning dot class at and above 70 (below 90)', () => {
    expect(getStatusDotClass(70)).toBe('bg-status-warning');
    expect(getStatusDotClass(89.99)).toBe('bg-status-warning');
  });

  it('returns the danger dot class at and above 90', () => {
    expect(getStatusDotClass(90)).toBe('bg-status-danger');
    expect(getStatusDotClass(100)).toBe('bg-status-danger');
  });
});
