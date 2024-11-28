import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';

// Create a client
const queryClient = new QueryClient();

const QueryProvider = ({ children }: { children: React.ReactNode }) => {
  return (
    // Provide the client to your App
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
};

export default QueryProvider;
