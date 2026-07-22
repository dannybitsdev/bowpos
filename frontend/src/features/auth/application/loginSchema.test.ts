import { describe, expect, it } from 'vitest';

import { loginSchema } from './loginSchema';

describe('loginSchema', () => {
  it('accepts non-empty passwords', () => {
    const result = loginSchema.safeParse({
      email: 'admin@example.com',
      password: 'admin123',
      tenantId: '',
    });

    expect(result.success).toBe(true);
  });

  it('rejects empty passwords', () => {
    const result = loginSchema.safeParse({
      email: 'admin@example.com',
      password: '',
      tenantId: '',
    });

    expect(result.success).toBe(false);
  });
});
