import React, { createContext, useContext, useEffect, useMemo, useState } from 'react';

interface ThemeColors {
  colorPrimary: string;
  colorSecondary: string;
  colorBackground: string;
  colorCardBg: string;
  colorSidebarBg: string;
  colorBorder: string;
  colorText: string;
  colorMuted: string;
  typography: string;
  logoUrl: string;
}

interface ThemeContextValue {
  theme: ThemeColors;
  updateTheme: (colors: Partial<ThemeColors>) => void;
}

const defaultTheme: ThemeColors = {
  colorPrimary: '#DEFF9A',
  colorSecondary: '#141414',
  colorBackground: '#0D0D0D',
  colorCardBg: '#141414',
  colorSidebarBg: '#0D0D0D',
  colorBorder: '#222222',
  colorText: '#F5F5F5',
  colorMuted: '#8B8B8B',
  typography: 'Inter, sans-serif',
  logoUrl: 'https://example.com/logo.png',
};

const ThemeContext = createContext<ThemeContextValue | undefined>(undefined);

export const ThemeProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [theme, setTheme] = useState<ThemeColors>(defaultTheme);

  useEffect(() => {
    const loadTheme = async () => {
      try {
        const response = await fetch(`${import.meta.env.VITE_API_URL ?? '/api'}/config/ui`);
        const data = await response.json();
        setTheme((current) => ({
          ...current,
          colorPrimary: data.primary_color ?? current.colorPrimary,
          colorSecondary: data.secondary_color ?? current.colorSecondary,
          colorBackground: data.background_color ?? current.colorBackground,
          typography: data.font_family ?? current.typography,
          logoUrl: data.logo_url ?? current.logoUrl,
        }));
      } catch (error) {
        console.warn('No se pudo cargar el tema', error);
      }
    };

    void loadTheme();
  }, []);

  useEffect(() => {
    document.documentElement.style.setProperty('--color-primary', theme.colorPrimary);
    document.documentElement.style.setProperty('--color-secondary', theme.colorSecondary);
    document.documentElement.style.setProperty('--color-background', theme.colorBackground);
    document.documentElement.style.setProperty('--color-card-bg', theme.colorCardBg);
    document.documentElement.style.setProperty('--color-sidebar-bg', theme.colorSidebarBg);
    document.documentElement.style.setProperty('--color-border', theme.colorBorder);
    document.documentElement.style.setProperty('--color-text', theme.colorText);
    document.documentElement.style.setProperty('--color-muted', theme.colorMuted);
    document.documentElement.style.setProperty('--font-family', theme.typography);
  }, [theme]);

  const updateTheme = (partial: Partial<ThemeColors>) => {
    setTheme((current) => ({ ...current, ...partial }));
  };

  const value = useMemo(() => ({ theme, updateTheme }), [theme]);

  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>;
};

export const useTheme = () => {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used inside ThemeProvider');
  }
  return context;
};
