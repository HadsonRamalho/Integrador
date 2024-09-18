import React from 'react';
import CPDF from './pdf_gen';

const ChamaContrato = ({ nomelocadora, cnpjLocadora, nomeAdmLocadora, numeroConta, numeroAgencia }) => (
  <div>
    <CPDF 
          nomelocadora={nomelocadora} 
          cnpjLocadora={cnpjLocadora} 
          nomeAdmLocadora={nomeAdmLocadora} 
          numeroConta={numeroConta} 
          numeroAgencia={numeroAgencia} 
      />
  </div>
);


export default ChamaContrato;
