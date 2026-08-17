import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { useAuthContext } from '../../application/AuthContext';
import { loginSchema } from '../../application/loginSchema';
import { loginRequest } from '../../infrastructure/http/authApi';

export function LoginPage() {
  const navigate = useNavigate();
  const { login } = useAuthContext();

  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [tenantId, setTenantId] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function onSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setError(null);

    const parsed = loginSchema.safeParse({ email, password, tenantId });
    if (!parsed.success) {
      setError('No fue posible iniciar sesión. Verifica los datos e intenta nuevamente.');
      return;
    }

    try {
      setLoading(true);
      const response = await loginRequest(parsed.data.email, parsed.data.password, parsed.data.tenantId || undefined);
      login(
        {
          accessToken: response.tokens.access_token,
          refreshToken: response.tokens.refresh_token,
        },
        response.user
      );
      navigate('/dashboard', { replace: true });
    } catch {
      setError('No fue posible iniciar sesión. Verifica los datos e intenta nuevamente.');
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="relative min-h-screen overflow-x-hidden bg-zinc-950 text-zinc-100">
      <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_top,_rgba(132,204,22,0.25),_transparent_45%),radial-gradient(circle_at_bottom_right,_rgba(234,88,12,0.2),_transparent_40%)]" />
      <div className="pointer-events-none absolute inset-0 opacity-20 [background-image:linear-gradient(rgba(255,255,255,.08)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,.08)_1px,transparent_1px)] [background-size:36px_36px]" />

      <main className="relative mx-auto flex min-h-screen w-full max-w-6xl items-center justify-center px-6 py-12">
        <section className="grid w-full min-w-0 overflow-hidden rounded-3xl border border-zinc-800 bg-zinc-900/80 shadow-2xl backdrop-blur md:grid-cols-2">
          <div className="hidden border-r border-zinc-800 bg-gradient-to-br from-zinc-900 via-zinc-950 to-zinc-900 p-10 md:block">
            <p className="text-xs uppercase tracking-[0.35em] text-lime-300">Sabor & Raiz</p>
            <h1 className="mt-4 text-4xl font-semibold leading-tight">Control seguro para operaciones POS multi-sede</h1>
            <p className="mt-6 text-sm leading-7 text-zinc-400">
              Acceso centralizado con políticas estrictas de roles y permisos por tenant.
            </p>
          </div>

          <form onSubmit={onSubmit} className="space-y-5 p-8 md:p-10">
            <h2 className="text-2xl font-semibold">Iniciar sesión</h2>
            <p className="text-sm text-zinc-400">Ingresa tus credenciales corporativas.</p>

            <label className="block text-sm">
              Correo
              <input
                className="mt-2 w-full rounded-xl border border-zinc-700 bg-zinc-950 px-3 py-2.5 outline-none transition focus:border-lime-300"
                type="email"
                value={email}
                onChange={(event) => setEmail(event.target.value)}
                autoComplete="email"
                required
              />
            </label>

            <label className="block text-sm">
              Contraseña
              <span className="relative mt-2 block">
                <input
                  className="w-full rounded-xl border border-zinc-700 bg-zinc-950 px-3 py-2.5 pr-12 outline-none transition focus:border-lime-300"
                  type={showPassword ? 'text' : 'password'}
                  value={password}
                  onChange={(event) => setPassword(event.target.value)}
                  autoComplete="current-password"
                  required
                />
                <button
                  type="button"
                  onClick={() => setShowPassword((visible) => !visible)}
                  className="absolute inset-y-0 right-0 flex w-12 items-center justify-center text-zinc-400 transition hover:text-lime-300"
                  aria-label={showPassword ? 'Ocultar contraseña' : 'Mostrar contraseña'}
                  aria-pressed={showPassword}
                >
                  <svg viewBox="0 0 24 24" className="h-5 w-5" fill="none" stroke="currentColor" strokeWidth="1.8" aria-hidden="true">
                    {showPassword ? <><path d="M3 3l18 18" /><path d="M10.6 10.6a2 2 0 0 0 2.8 2.8" /><path d="M9.9 5.2A10.7 10.7 0 0 1 12 5c5 0 8.5 4.5 9.5 7a15 15 0 0 1-3.1 4.4M6.2 6.2C3.8 7.7 2.7 10 2.5 12c1 2.5 4.5 7 9.5 7 1.3 0 2.5-.3 3.6-.8" /></> : <><path d="M2.5 12S6 5 12 5s9.5 7 9.5 7-3.5 7-9.5 7-9.5-7-9.5-7Z" /><circle cx="12" cy="12" r="2.5" /></>}
                  </svg>
                </button>
              </span>
            </label>

            <label className="block text-sm">
              Tenant ID (opcional)
              <input
                className="mt-2 w-full rounded-xl border border-zinc-700 bg-zinc-950 px-3 py-2.5 outline-none transition focus:border-lime-300"
                type="text"
                value={tenantId}
                onChange={(event) => setTenantId(event.target.value)}
                placeholder="UUID del tenant"
              />
            </label>

            {error ? <p className="rounded-xl border border-rose-400/40 bg-rose-400/10 px-3 py-2 text-sm">{error}</p> : null}

            <button
              type="submit"
              disabled={loading}
              className="w-full rounded-xl bg-lime-300 px-4 py-3 font-semibold text-zinc-950 transition hover:bg-lime-200 disabled:cursor-not-allowed disabled:opacity-70"
            >
              {loading ? 'Validando...' : 'Entrar'}
            </button>
          </form>
        </section>
      </main>
    </div>
  );
}
