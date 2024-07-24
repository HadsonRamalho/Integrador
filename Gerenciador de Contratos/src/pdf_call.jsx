import React, { useState, useEffect } from 'react';
import CreatePdf from './pdf_gen';
import './styles.css';

function CPDF() {
  const [pdfUrls, setPdfUrls] = useState([null, null, null]);

  useEffect(() => {
    async function generatePdfs() {
      try {
        const pdfPromises = [
          await CreatePdf({
            nomeLocadora: "TechSolutions Ltda.",
            cnpjLocadora: "12.345.678/0001-90",
            nomeAdmLocadora: "João Silva",
            cpfAdmLocadora: "123.456.789-00",
            enderecoAdmLocadora: "Rua das Dores, 321",
            enderecoLocadora: "Rua das Flores, 123",
            cidadeLocadora: "São Paulo",
            estadoLocadora: "SP"
          })
        ];

        const pdfBytesArray = await Promise.all(pdfPromises);
        const urls = pdfBytesArray.map(pdfBytes => {
          const blob = new Blob([pdfBytes], { type: 'application/pdf' });
          return URL.createObjectURL(blob);
        });

        setPdfUrls(urls);
      } catch (error) {
        console.error('Failed to create PDFs', error);
      }
    }

    generatePdfs();
  }, []);

  return (
    <div className="App">
      <table className="pdf-table">
        {pdfUrls.map((pdfUrl, index) => (
          <td key={index} className="pdf-cell">
            {pdfUrl ? (
              <iframe
                src={pdfUrl}
                width="300%"
                height="500px"
                title={`PDF Viewer ${index + 1}`}
              ></iframe>
            ) : (
              <p>Carregando PDF {index + 1}...</p>
            )}
          </td>
        ))}
      </table>
    </div>
  );
}

export default CPDF;
