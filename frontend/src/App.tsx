import { BrowserRouter } from 'react-router-dom';

import { AppRouter } from './app/AppRouter';
import { AuthProvider } from './features/auth/application/AuthContext';
import { ThemeProvider } from './context/ThemeContext';

function App() {
  return (
    <ThemeProvider>
      <AuthProvider>
        <BrowserRouter>
          <AppRouter />
        </BrowserRouter>
      </AuthProvider>
    </ThemeProvider>
  );
}

export default App;
