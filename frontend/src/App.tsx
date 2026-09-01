import { BrowserRouter } from 'react-router-dom';

import { AppRouter } from './app/AppRouter';
import { AuthProvider } from './features/auth/application/AuthContext';
import { ThemeProvider } from './context/ThemeContext';
import { ConfirmationModalHost } from './features/modal/presentation/ConfirmationModalHost';

function App() {
  return (
    <ThemeProvider>
      <AuthProvider>
        <BrowserRouter>
          <AppRouter />
          <ConfirmationModalHost />
        </BrowserRouter>
      </AuthProvider>
    </ThemeProvider>
  );
}

export default App;
