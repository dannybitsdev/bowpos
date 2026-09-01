import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { OrderBuilderPage } from './OrderBuilderPage';

vi.mock('../features/auth/infrastructure/http/apiClient', () => ({
  default: { get: vi.fn().mockResolvedValue({ data: [] }), post: vi.fn() },
}));

describe('OrderBuilderPage layout', () => {
  it('keeps the header, filters bar and order summary as fixed, non-scrolling wrappers while the menu grid scrolls independently', async () => {
    render(<OrderBuilderPage />);

    const header = await screen.findByTestId('order-fixed-header');
    const filtersBar = screen.getByTestId('order-filters-bar');
    const scrollableMenu = screen.getByTestId('order-scrollable-menu');
    const summaryPanel = screen.getByTestId('order-summary-panel');

    // Fixed regions must not carry their own scroll/overflow behavior.
    expect(header.className).not.toMatch(/overflow-y-auto/);
    expect(filtersBar.className).not.toMatch(/overflow-y-auto/);
    expect(header.className).toContain('shrink-0');
    expect(filtersBar.className).toContain('shrink-0');

    // Only the menu grid is scrollable.
    expect(scrollableMenu.className).toContain('overflow-y-auto');

    // The order summary panel is a fixed column, not part of the scrollable menu.
    expect(summaryPanel).not.toContainElement(scrollableMenu);
    expect(scrollableMenu).not.toContainElement(summaryPanel);

    // The outer viewport is a fixed, non-scrolling grid (100vh style layout).
    const viewport = header.closest('section');
    expect(viewport?.className).toContain('h-screen');
    expect(viewport?.className).toContain('overflow-hidden');
  });
});
