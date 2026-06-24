import { describe, it, expect } from 'vitest';
import { getStatusColor, getStatusClass, getStatusDotClass } from './colors';

// These thresholds drive the UI's risk signaling: a percentage >= 90 is
// treated as danger (critical balance/usage), >= 70 as warning, otherwise ok.
// A regression in any boundary would silently hide critical states from
// users, so each threshold is pinned at its exact boundary.

describe('getStatusColor', () => {
  it('returns danger red at and above 90%', () => {
    expect(getStatusColor(90)).toBe('#EF4444');
    expect(getStatusColor(100)).toBe('#EF4444');
  });

  it('returns warning amber between 70 (inclusive) and 90 (exclusive)', () => {
    expect(getStatusColor(89.99)).toBe('#F59E0B');
    expect(getStatusColor(70)).toBe('#F59E0B');
  });

  it('returns ok green below 70%', () => {
    expect(getStatusColor(69.99)).toBe('#34c759');
    expect(getStatusColor(0)).toBe('#34c759');
    expect(getStatusColor(-1)).toBe('#34c759');
  });
});

describe('getStatusClass', () => {
  it('returns the danger class at and above 90%', () => {
    expect(getStatusClass(90)).toBe('text-status-danger');
    expect(getStatusClass(100)).toBe('text-status-danger');
  });

  it('returns the warning class between 70 (inclusive) and 90 (exclusive)', () => {
    expect(getStatusClass(89.99)).toBe('text-status-warning');
    expect(getStatusClass(70)).toBe('text-status-warning');
  });

  it('returns the ok class below 70%', () => {
    expect(getStatusClass(69.99)).toBe('text-status-ok');
    expect(getStatusClass(0)).toBe('text-status-ok');
  });
});

describe('getStatusDotClass', () => {
  it('returns the danger dot class at and above 90%', () => {
    expect(getStatusDotClass(90)).toBe('bg-status-danger');
    expect(getStatusDotClass(100)).toBe('bg-status-danger');
  });

  it('returns the warning dot class between 70 (inclusive) and 90 (exclusive)', () => {
    expect(getStatusDotClass(89.99)).toBe('bg-status-warning');
    expect(getStatusDotClass(70)).toBe('bg-status-warning');
  });

  it('returns the ok dot class below 70%', () => {
    expect(getStatusDotClass(69.99)).toBe('bg-status-ok');
    expect(getStatusDotClass(0)).toBe('bg-status-ok');
  });
});
