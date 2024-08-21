import React from 'react';
import ReactDOM from 'react-dom/client';
import CPDF from './pdf_call';
import './styles.css';

//localStorage.removeItem('token');

const rootElement = document.getElementById('pdf');
if (rootElement) {
  ReactDOM.createRoot(rootElement).render(
      <CPDF />
  );
}
