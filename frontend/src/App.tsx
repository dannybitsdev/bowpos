import React from 'react';
import { Sidebar } from './components/Sidebar';
import { Dashboard } from './pages/Dashboard';
import { ThemeProvider } from './context/ThemeContext';

const App: React.FC = () => {
  return (
    <ThemeProvider>
      <div className="flex min-h-screen bg-[var(--color-background)]">
        <Sidebar />
        <main className="flex-1">
          <Dashboard />
        </main>
      </div>
    </ThemeProvider>
  );
};

export default App;
