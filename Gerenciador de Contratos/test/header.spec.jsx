import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';
import Header from "../src/components/header";
import "@testing-library/jest-dom/vitest";

describe('Header Component', () => {
  it("deve renderizar corretamente o header da página.", () => {
    render(
      <BrowserRouter>
        <Header />
      </BrowserRouter>
    );
    expect(screen.getByRole('heading')).toHaveTextContent("header de página.");
  });
});