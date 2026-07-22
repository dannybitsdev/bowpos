import { z } from 'zod';

export const loginSchema = z.object({
  email: z.string().trim().email('Correo inválido'),
  password: z.string().min(1, 'Contraseña requerida'),
  tenantId: z.string().uuid('Tenant inválido').optional().or(z.literal('')),
});

export type LoginInput = z.infer<typeof loginSchema>;
