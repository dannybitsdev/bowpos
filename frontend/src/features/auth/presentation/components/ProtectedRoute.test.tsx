import { render, screen } from '@testing-library/react';
import { MemoryRouter, Route, Routes } from 'react-router-dom';
import { beforeEach, describe, expect, it } from 'vitest';

import { useAuthStore } from '../../application/authStore';
import { ProtectedRoute } from './ProtectedRoute';

describe('ProtectedRoute', () => {
  beforeEach(() => {
    useAuthStore.getState().logout();
  });

  it('blocks unauthenticated users', () => {
    render(
      <MemoryRouter initialEntries={['/private']}>
        <Routes>
          <Route path="/login" element={<p>Login page</p>} />
          <Route
            path="/private"
            element={
              <ProtectedRoute allowedRoles={['SUPER_ADMIN']}>
                <p>Private content</p>
              </ProtectedRoute>
            }
          />
        </Routes>
      </MemoryRouter>
    );

    expect(screen.getByText('Login page')).toBeInTheDocument();
  });

  it('renders content for allowed roles', () => {
    useAuthStore.getState().login({ accessToken: 'token', refreshToken: 'refresh' }, {
      user_id: 'u1',
      tenant_id: 't1',
      role: 'SUPER_ADMIN',
      permissions: ['manage:tenant_admins'],
    });

    render(
      <MemoryRouter initialEntries={['/private']}>
        <Routes>
          <Route
            path="/private"
            element={
              <ProtectedRoute allowedRoles={['SUPER_ADMIN']}>
                <p>Private content</p>
              </ProtectedRoute>
            }
          />
          <Route path="/unauthorized" element={<p>Unauthorized</p>} />
        </Routes>
      </MemoryRouter>
    );

    expect(screen.getByText('Private content')).toBeInTheDocument();
  });
});
