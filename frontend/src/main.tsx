import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './index.css';
import { RouterProvider } from 'react-router-dom';
import router from './router.tsx';
import Providers from './providers/Providers.tsx';
import { ErrorBoundary } from 'react-error-boundary';
import { MainFallback } from './routes/fallbacks/MainFallback.tsx';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Providers>
      <ErrorBoundary FallbackComponent={MainFallback}>
        <RouterProvider router={router} />
      </ErrorBoundary>
    </Providers>
  </StrictMode>
);
