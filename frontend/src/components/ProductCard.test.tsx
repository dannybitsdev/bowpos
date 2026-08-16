import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';

import { ProductCard } from './ProductCard';

describe('ProductCard', () => {
  it('uses a lazy image and fluid responsive card constraints', () => {
    const { container } = render(<ProductCard product={{ id: '1', category_id: '1', name: 'Arepa', description: 'Maíz', price: 5000, stock: 10, image_url: 'https://example.com/arepa.jpg' }} />);

    expect(screen.getByRole('img', { name: 'Arepa' })).toHaveAttribute('loading', 'lazy');
    expect(container.firstElementChild).toHaveClass('min-w-0');
    expect(container.querySelector('[class*="aspect-"]')).toBeInTheDocument();
  });
});