import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';

import { ProductCard } from './ProductCard';

describe('ProductCard', () => {
  it('uses a lazy image and fluid responsive card constraints', () => {
    const { container } = render(<ProductCard product={{ id: '1', category_id: '1', name: 'Arepa', description: 'Maíz', price: 5000, stock: 10, image_url: 'https://example.com/arepa.jpg' }} />);

    expect(screen.getByRole('img', { name: 'Arepa' })).toHaveAttribute('loading', 'lazy');
    expect(container.firstElementChild).toHaveClass('min-w-0');
    expect(container.firstElementChild).toHaveClass('h-full');
    expect(container.querySelector('[class*="aspect-"]')).toBeInTheDocument();
  });

  it('renders a placeholder when the product has no image', () => {
    render(<ProductCard product={{ id: '2', category_id: '1', name: 'Jugo', price: 3000, stock: 5, image_url: null }} />);

    expect(screen.queryByRole('img')).not.toBeInTheDocument();
    expect(screen.getByText('Sin imagen')).toBeInTheDocument();
  });

  it('falls back to a placeholder when the image fails to load', () => {
    render(<ProductCard product={{ id: '3', category_id: '1', name: 'Postre', price: 4000, stock: 3, image_url: 'https://example.com/broken.jpg' }} />);

    const image = screen.getByRole('img', { name: 'Postre' });
    fireEvent.error(image);

    expect(screen.queryByRole('img')).not.toBeInTheDocument();
    expect(screen.getByText('Imagen no disponible')).toBeInTheDocument();
  });

  it('keeps a strict aspect-ratio image container with object-cover to avoid distortion', () => {
    const { container } = render(<ProductCard product={{ id: '4', category_id: '1', name: 'Torta', price: 6000, stock: 1, image_url: 'https://example.com/torta.jpg' }} />);

    const imageWrapper = container.querySelector('[class*="aspect-"]');
    expect(imageWrapper?.className).toMatch(/aspect-square|aspect-\[4\/3\]/);
    expect(screen.getByRole('img', { name: 'Torta' })).toHaveClass('object-cover');
  });
});