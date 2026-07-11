import React from 'react';

type Metric = {
  label: string;
  value: string;
  change: string;
  accent: string;
  icon: React.ReactNode;
};

type ComplianceItem = {
  title: string;
  status: 'Vigente' | 'Por vencer' | 'Vencido';
  detail: string;
};

const metrics: Metric[] = [
  { label: 'Ventas Totales', value: '$24.500.000', change: '+12.3% vs mes anterior', accent: 'text-emerald-400', icon: <DollarIcon /> },
  { label: 'Costo', value: '$8.200.000', change: '+4.8%', accent: 'text-amber-400', icon: <CostIcon /> },
  { label: 'Utilidad Bruta', value: '$16.300.000', change: '+8.7%', accent: 'text-cyan-400', icon: <ProfitIcon /> },
  { label: 'Órdenes', value: '1,240', change: '+9.1%', accent: 'text-violet-400', icon: <OrdersIcon /> },
  { label: 'Ticket Promedio', value: '$19.750', change: '+6.2%', accent: 'text-rose-400', icon: <TicketIcon /> },
];

const complianceItems: ComplianceItem[] = [
  { title: 'Matrícula Mercantil (Cámara de Comercio)', status: 'Vigente', detail: 'Actualizada' },
  { title: 'Concepto Sanitario (Secretaría de Salud)', status: 'Vigente', detail: 'Sin observaciones' },
  { title: 'Sayco & Acinpro', status: 'Por vencer', detail: '15 días' },
  { title: 'Curso de Manipulación de Alimentos', status: 'Vencido', detail: '1 alerta personal' },
  { title: 'Extintores y Plan de Emergencia', status: 'Vigente', detail: 'Verificado' },
];

const salesSeries = [42, 58, 46, 78, 72, 88, 94];
const categories = [
  { label: 'Platillos', value: 58, color: 'var(--color-primary)' },
  { label: 'Bebidas', value: 24, color: '#6b7280' },
  { label: 'Postres', value: 18, color: '#374151' },
];

function DollarIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="M12 3v18M15.5 6a3.5 3.5 0 0 0-7 0c0 2.2 1.6 3.2 3.5 4 2.2.8 3.5 1.8 3.5 3.8a3.5 3.5 0 0 1-7 0" />
    </svg>
  );
}

function CostIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="M5 18V8m7 10V5m7 13v-7" />
    </svg>
  );
}

function ProfitIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="M4 16 10 10l3 3 7-7" />
      <path d="M14 6h6v6" />
    </svg>
  );
}

function OrdersIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <rect x="4" y="5" width="16" height="14" rx="2" />
      <path d="M8 10h8M8 14h5" />
    </svg>
  );
}

function TicketIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="M5 7h14M5 12h14M5 17h14" />
    </svg>
  );
}

function StatusBadge({ status }: { status: ComplianceItem['status'] }) {
  const styles = {
    Vigente: 'border-emerald-500/30 bg-emerald-500/10 text-emerald-300',
    'Por vencer': 'border-amber-500/30 bg-amber-500/10 text-amber-300',
    Vencido: 'border-rose-500/30 bg-rose-500/10 text-rose-300',
  } as const;

  return <span className={`rounded-full border px-2.5 py-1 text-[11px] font-medium ${styles[status]}`}>{status}</span>;
}

export const Dashboard: React.FC = () => {
  return (
    <div className="min-h-screen bg-[var(--color-background)] p-4 text-[var(--color-text)] lg:p-8">
      <div className="mb-6 flex flex-wrap items-center justify-between gap-4">
        <div>
          <p className="text-[11px] font-semibold uppercase tracking-[0.35em] text-[var(--color-primary)]">Panel Central</p>
          <h1 className="text-3xl font-semibold tracking-tight text-black">Callejeros</h1>
        </div>
        <div className="rounded-2xl border border-[var(--color-border)] bg-[var(--color-card-bg)] px-4 py-3 text-sm text-[var(--color-muted)] shadow-panel">
          Hoy • 08/07/2026
        </div>
      </div>

      <div className="mb-6 grid gap-4 md:grid-cols-2 xl:grid-cols-5">
        {metrics.map((metric) => (
          <div key={metric.label} className="rounded-2xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-4 shadow-panel">
            <div className="mb-4 flex items-center justify-between">
              <p className="text-sm text-[var(--color-muted)]">{metric.label}</p>
              <span className="rounded-full bg-black/30 p-2 text-[var(--color-primary)]">{metric.icon}</span>
            </div>
            <div className="space-y-2">
              <p className="text-2xl font-semibold text-white">{metric.value}</p>
              <p className={`text-sm font-medium ${metric.accent}`}>{metric.change}</p>
            </div>
          </div>
        ))}
      </div>

      <div className="grid gap-6 xl:grid-cols-[2fr_1fr]">
        <div className="space-y-6">
          <div className="rounded-3xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-5 shadow-panel">
            <div className="mb-4 flex items-center justify-between">
              <div>
                <h2 className="text-lg font-semibold text-white">Ventas vs Costos</h2>
                <p className="text-sm text-[var(--color-muted)]">Rendimiento semanal</p>
              </div>
              <span className="rounded-full border border-emerald-500/20 bg-emerald-500/10 px-3 py-1 text-sm font-medium text-emerald-300">
                +14.2%
              </span>
            </div>

            <div className="rounded-2xl border border-[var(--color-border)] bg-[#0D0D0D] p-4">
              <svg viewBox="0 0 420 220" className="h-56 w-full" role="img" aria-label="Ventas vs Costos">
                <line x1="24" y1="188" x2="396" y2="188" stroke="#2b2b2b" strokeWidth="1" />
                <line x1="24" y1="140" x2="396" y2="140" stroke="#2b2b2b" strokeWidth="1" />
                <line x1="24" y1="92" x2="396" y2="92" stroke="#2b2b2b" strokeWidth="1" />
                <line x1="24" y1="44" x2="396" y2="44" stroke="#2b2b2b" strokeWidth="1" />
                <path d="M24 160 C 72 152, 100 148, 128 126 S 196 94, 224 104 S 288 140, 324 122 S 372 84, 396 64" fill="none" stroke="var(--color-primary)" strokeWidth="3" strokeLinecap="round" />
                <path d="M24 172 C 72 166, 100 174, 128 158 S 196 136, 224 142 S 288 170, 324 154 S 372 126, 396 118" fill="none" stroke="#6b7280" strokeWidth="3" strokeLinecap="round" />
                {salesSeries.map((value, index) => {
                  const x = 24 + index * 62;
                  const y = 188 - value;
                  return <circle key={`${value}-${index}`} cx={x} cy={y} r="4.5" fill="var(--color-primary)" />;
                })}
              </svg>
              <div className="mt-3 flex gap-6 text-sm text-[var(--color-muted)]">
                <span className="flex items-center gap-2"><span className="h-2.5 w-2.5 rounded-full bg-[var(--color-primary)]" /> Ventas</span>
                <span className="flex items-center gap-2"><span className="h-2.5 w-2.5 rounded-full bg-gray-500" /> Costos</span>
              </div>
            </div>
          </div>

          <div className="grid gap-6 lg:grid-cols-[1.2fr_0.8fr]">
            <div className="rounded-3xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-5 shadow-panel">
              <h2 className="mb-4 text-lg font-semibold text-white">Ventas por categoría</h2>
              <div className="flex items-center justify-center">
                <div className="relative flex h-44 w-44 items-center justify-center rounded-full">
                  <svg viewBox="0 0 120 120" className="h-44 w-44 -rotate-90">
                    <circle cx="60" cy="60" r="42" stroke="#1f2937" strokeWidth="16" fill="none" />
                    <circle cx="60" cy="60" r="42" stroke="var(--color-primary)" strokeWidth="16" fill="none" strokeDasharray="264" strokeDashoffset="110" strokeLinecap="round" />
                  </svg>
                  <div className="absolute text-center">
                    <p className="text-3xl font-semibold text-white">58%</p>
                    <p className="text-sm text-[var(--color-muted)]">Platillos</p>
                  </div>
                </div>
              </div>
              <div className="mt-4 space-y-2">
                {categories.map((category) => (
                  <div key={category.label} className="flex items-center justify-between text-sm text-[var(--color-muted)]">
                    <span className="flex items-center gap-2">
                      <span className="h-2.5 w-2.5 rounded-full" style={{ backgroundColor: category.color }} />
                      {category.label}
                    </span>
                    <span className="font-medium text-white">{category.value}%</span>
                  </div>
                ))}
              </div>
            </div>

            <div className="rounded-3xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-5 shadow-panel">
              <p className="text-sm text-[var(--color-muted)]">Plato más vendido</p>
              <h3 className="mt-2 text-xl font-semibold text-white">Bandeja Paisa</h3>
              <p className="mt-3 text-sm leading-6 text-[var(--color-muted)]">
                El plato de mayor demanda en la sede principal con un volumen de 342 unidades esta semana.
              </p>
              <div className="mt-4 rounded-2xl border border-[var(--color-border)] bg-[#0D0D0D] p-4">
                <p className="text-3xl font-semibold text-[var(--color-primary)]">342</p>
                <p className="text-sm text-[var(--color-muted)]">unidades vendidas</p>
              </div>
            </div>
          </div>
        </div>

        <div className="rounded-3xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-5 shadow-panel">
          <div className="mb-5 flex items-center justify-between">
            <div>
              <h2 className="text-lg font-semibold text-white">Cumplimiento Legal & Alertas</h2>
              <p className="text-sm text-[var(--color-muted)]">Control operativo del restaurante</p>
            </div>
            <span className="rounded-full border border-amber-500/20 bg-amber-500/10 px-3 py-1 text-sm font-medium text-amber-300">
              2 alertas
            </span>
          </div>

          <div className="space-y-3">
            {complianceItems.map((item) => (
              <div key={item.title} className="rounded-2xl border border-[var(--color-border)] bg-[#0D0D0D] p-3">
                <div className="flex items-start justify-between gap-3">
                  <div>
                    <p className="text-sm font-medium text-white">{item.title}</p>
                    <p className="mt-1 text-xs text-[var(--color-muted)]">{item.detail}</p>
                  </div>
                  <StatusBadge status={item.status} />
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};
