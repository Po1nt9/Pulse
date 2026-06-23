import { describe, it, expect } from 'vitest';
import {
  getStatusColor,
  getStatusClass,
  getStatusDotClass,
} from './colors';

// These three functions share identical threshold semantics (>=90 danger,
// >=70 warning, otherwise ok). They drive the user-facing status indicator
// across ProviderCard / StatusIndicator / BalanceDisplay, so a boundary
// regression would silently show the wrong health state for a provider.

describe('getStatusColor', () => {
  it('returns the danger color at and above 90', () => {
    expect(getStatusColor(90)).toBe('#EF4444');
    expect(getStatusColor(95)).toBe('#EF4444');
    expect(getStatusColor(100)).toBe('#EF4444');
  });

  it('returns the warning color in [70, 90)', () => {
    expect(getStatusColor(70)).toBe('#F59E0B');
    expect(getStatusColor(89)).toBe('#F59E0B');
    expect(getStatusColor(89.99)).toBe('#F59E0B');
  });

  it('returns the ok color below 70', () => {
    expect(getStatusColor(69.99)).toBe('#34c759');
    expect(getStatusColor(0)).toBe('#34c759');
  });

  it('treats the 90 boundary as danger, not warning', () => {
    // Regression guard: 90 must be danger (>=90), 89.99 must stay warning.
    expect(getStatusColor(90)).not.toBe('#F59E0B');
    expect(getStatusColor(89.99)).not.toBe('#EF4444');
  });
});

describe('getStatusClass', () => {
  it('maps thresholds to text status classes', () => {
    expect(getStatusClass(90)).toBe('text-status-danger');
    expect(getStatusClass(70)).toBe('text-status-warning');
    expect(getStatusClass(69.99)).toBe('text-status-ok');
  });
});

describe('getStatusDotClass', () => {
  it('maps thresholds to background status classes', () => {
    expect(getStatusDotClass(90)).toBe('bg-status-danger');
    expect(getStatusDotClass(70)).toBe('bg-status-warning');
    expect(getStatusDotClass(69.99)).toBe('bg-status-ok');
  });
});
