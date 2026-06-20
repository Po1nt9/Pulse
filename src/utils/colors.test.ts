import { describe, it, expect } from 'vitest';
import { getStatusColor, getStatusClass, getStatusDotClass } from './colors';

// These three functions share the same threshold contract that drives the
// provider health indicator across the UI (StatusIndicator, ProviderCard,
// OverviewPanel). An off-by-one regression here would misclassify a
// provider as healthy when it is in fact degraded, so the boundary values
// 70 and 90 are the focus of this suite.

describe('getStatusColor', () => {
  it('returns green for a healthy provider well below the warning band', () => {
    expect(getStatusColor(0)).toBe('#34c759');
    expect(getStatusColor(50)).toBe('#34c759');
  });

  it('stays green just below the 70 warning threshold', () => {
    expect(getStatusColor(69.99)).toBe('#34c759');
  });

  it('switches to amber exactly at 70', () => {
    expect(getStatusColor(70)).toBe('#F59E0B');
  });

  it('stays amber just below the 90 danger threshold', () => {
    expect(getStatusColor(89.99)).toBe('#F59E0B');
  });

  it('switches to red exactly at 90', () => {
    expect(getStatusColor(90)).toBe('#EF4444');
  });

  it('stays red at and above 100', () => {
    expect(getStatusColor(100)).toBe('#EF4444');
    expect(getStatusColor(150)).toBe('#EF4444');
  });

  it('treats negative usage as healthy (green)', () => {
    // Negative percentage (e.g. credit balance) must not trip the warning.
    expect(getStatusColor(-5)).toBe('#34c759');
  });
});

describe('getStatusClass', () => {
  it('returns the ok class below 70', () => {
    expect(getStatusClass(0)).toBe('text-status-ok');
    expect(getStatusClass(69.99)).toBe('text-status-ok');
  });

  it('returns the warning class at and above 70 up to 89.99', () => {
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

  it('returns the warning dot class at and above 70 up to 89.99', () => {
    expect(getStatusDotClass(70)).toBe('bg-status-warning');
    expect(getStatusDotClass(89.99)).toBe('bg-status-warning');
  });

  it('returns the danger dot class at and above 90', () => {
    expect(getStatusDotClass(90)).toBe('bg-status-danger');
    expect(getStatusDotClass(100)).toBe('bg-status-danger');
  });
});
