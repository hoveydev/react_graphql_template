import { createBrowserRouter } from 'react-router-dom';

import App from './App';
import { ErrorView } from './pages/Error/View';

export const router = createBrowserRouter([
  {
    path: '/',
    element: <App />,
    errorElement: <ErrorView />
  }
]);
