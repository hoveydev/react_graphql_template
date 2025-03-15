import { StrictMode } from 'react';

import { ApolloClient, InMemoryCache, ApolloProvider } from '@apollo/client';
import { createRoot } from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';

import { router } from './router.tsx';

import './index.css';

const client = new ApolloClient({
  uri: import.meta.env.VITE_GRAPHQL_ENDPOINT,
  cache: new InMemoryCache()
});

createRoot(document.getElementById('root')!).render(
  <ApolloProvider client={client}>
    <StrictMode>
      <RouterProvider router={router} />
    </StrictMode>
  </ApolloProvider>
);
