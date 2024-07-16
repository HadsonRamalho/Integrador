import React from 'react';
import ReactDOM from 'react-dom/client';
import CPDF from './pdf_call';
import './styles.css';

const rootElement = document.getElementById('pdf');
if (rootElement) {
  ReactDOM.createRoot(rootElement).render(
    <React.StrictMode>
      <CPDF />
    </React.StrictMode>
  );
}
