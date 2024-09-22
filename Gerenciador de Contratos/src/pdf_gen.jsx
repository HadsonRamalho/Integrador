import React from 'react';
import { Page, Text, View, Document, StyleSheet, PDFViewer } from '@react-pdf/renderer';
import { useLocation } from 'react-router-dom';

// Estilos
const styles = StyleSheet.create({
  page: { flexDirection: 'column', padding: 30 },
  section: { margin: 10, padding: 10, fontSize: 12 },
  title: { fontSize: 18, textAlign: 'center', marginBottom: 10 },
  table: { display: 'table', width: 'auto', margin: '10px 0' },
  tableRow: { flexDirection: 'row' },
  tableCol: { width: '25%', borderStyle: 'solid', borderWidth: 1, borderColor: '#000' },
  tableCell: { margin: 5, fontSize: 10 },
});

// Função para gerar o contrato com base nos parâmetros fornecidos
export const gerarTextoContratoP1 = ({
  nomeLocadora,
  cnpjLocadora,
  logradouro,
  numeroendereco,
  complemento,
  cidade,
  uf,
  nomeAdmLocadora,
  cpf,
  orgaoemissor,
  nacionalidade,
  estadocivil,
  logradouroadm,
  cepadm ,
  ufadm ,
  complementoadm,
  cidadeadm ,
  numeroenderecoadm,
}) => {
  return `
    LOCADORA: ${nomeLocadora}, inscrita no CNPJ sob o nº ${cnpjLocadora}, com sede em
${logradouro}, N° ${numeroendereco}, ${complemento},  ${cidade}/${uf}, neste ato representada pelo seu sócio administrador, ${nomeAdmLocadora}, ${nacionalidade}, ${estadocivil}, inscrito no CPF sob nº ${cpf} emitido por ${orgaoemissor}, com endereço em ${cepadm}, ${logradouroadm}, ${numeroenderecoadm}, ${complementoadm}, ${cidadeadm}/${ufadm}.
LOCATÁRIA: XXXXXXXXXX, inscrita no CNPJ sob o nº XXXXXXXXXXX, com sede na Rua xxxxxxxxx, Cidade/MG, representada por XXXXXXXXXXXX, brasileiro, casado, CPF XXXXXXXXX, residente na Rua XXXXXXXXXXX, Cidade/MG.
DEFINIÇÕES

As expressões grafadas em “caixa alta” neste contrato têm os seguintes significados:

LOCADORA: Pessoa Jurídica que disponibiliza o(s) bem(ns) em locação.
LOCATÁRIA: Pessoa Jurídica que recebe o(s) bem(ns) em locação.
PARTES: Referem-se à LOCADORA e LOCATÁRIA conjuntamente.
PARTE: Refere-se à LOCADORA ou LOCATÁRIA isoladamente.

CLÁUSULA PRIMEIRA – DO OBJETO, PRAZO E USO

1.1 A LOCADORA cede à LOCATÁRIA os seguintes bens móveis listados abaixo, declarando-se legítima possuidora e/ou proprietária:
  `;
};

const gerarTextoContratoP2 = ({
}) => {
  return `
    1.1.1 Os equipamentos serão retirados no endereço especificado pela LOCADORA.

1.1.2 Os equipamentos serão utilizados pela LOCATÁRIA no endereço mencionado, com todas as responsabilidades sobre seu uso, guarda e depósito.

1.1.3 Os equipamentos serão mantidos sob a guarda da LOCATÁRIA como fiel depositária, assumindo todas as responsabilidades conforme a lei civil e penal.

CLÁUSULA SEGUNDA – PRAZO DE VIGÊNCIA DO CONTRATO

2.1 O prazo da locação é de 03 (três) meses a partir da data de entrega dos equipamentos, em 23 de novembro de 2023.

2.2 O contrato pode ser renovado por igual período mediante acordo entre as partes, com ajuste do valor conforme Cláusula Terceira.

CLÁUSULA TERCEIRA – DO VALOR E FORMA DE PAGAMENTO

3.1 A LOCATÁRIA pagará à LOCADORA R$ 19.000,00 mensais, ajustáveis anualmente pelo IPCA ou índice equivalente, adiantados até o dia 23 de cada mês.

3.1.1 A primeira parcela (entrada) será paga por transferência bancária conforme dados fornecidos pela LOCADORA.

3.1.2 Atrasos incorrerão em multa de 2% e juros de 1% ao mês sobre o débito corrigido.

3.1.3 O valor acordado cobre o uso de 200 horas/mês; horas adicionais serão cobradas separadamente.

CLÁUSULA QUARTA – ESTADO DE CONSERVAÇÃO

4.1 Os equipamentos estão no estado descrito nos laudos de entrega, reconhecido por ambas as partes.

CLÁUSULA QUINTA – UTILIZAÇÃO E MANUTENÇÃO

5.1 A LOCATÁRIA usará os bens conforme suas especificações técnicas e normas de segurança.

5.2 A LOCADORA é responsável pelas manutenções preventivas; danos por uso normal são de sua responsabilidade.

5.3 Danos por mau uso são responsabilidade da LOCATÁRIA, exceto os dente da escavadeira.

5.4 Defeitos pré-existentes serão corrigidos pela LOCADORA.

5.5 Substituições de peças devem ser feitas com autorização da LOCADORA.

5.6 Adaptar ou instalar peças sem autorização é proibido.

5.7 A LOCADORA pode inspecionar os bens a qualquer momento.

5.8 A LOCATÁRIA mantém sigilo sobre informações confidenciais e protegidas por patente.

5.9 Mudanças nos bens devem ser comunicadas à LOCADORA.

CLÁUSULA SEXTA – RESPONSABILIDADES

6.1 Acidentes e danos durante o uso são responsabilidade da LOCATÁRIA, salvo culpa comprovada da LOCADORA.

6.2 Uso inadequado pode resultar em rescisão do contrato pela LOCADORA.

6.3 Danos a terceiros são responsabilidade exclusiva da LOCATÁRIA, incluindo despesas legais.

6.4 A LOCATÁRIA informará à LOCADORA sobre reclamações legais relacionadas aos bens.

6.5 Normas ambientais são responsabilidade da LOCATÁRIA.

6.6 A LOCATÁRIA deve contratar pessoal técnico para operar os equipamentos.

CLÁUSULA SÉTIMA – DEVOLUÇÃO DOS EQUIPAMENTOS

7.1 A devolução dos equipamentos será no endereço da LOCADORA em Diamantina.

7.2 Os equipamentos serão vistoriados pela LOCADORA no ato da devolução.

7.3 Danos além do desgaste normal devem ser reparados pela LOCATÁRIA antes da devolução.

CLÁUSULA OITAVA – EXTRAVIO E DANOS TOTAIS

8.1 Danos totais serão indenizados pela LOCATÁRIA pelo valor de mercado.

8.2 A LOCATÁRIA é responsável por extravios durante a locação.

CLÁUSULA NONA – TRIBUTAÇÃO

9.1 Tributos são de responsabilidade da LOCADORA, sem ônus para a LOCATÁRIA.

CLÁUSULA DÉCIMA – SUB-LOCAÇÃO

10.1 Sub-locação dentro do grupo econômico da LOCATÁRIA requer autorização da LOCADORA.

CLÁUSULA DÉCIMA PRIMEIRA – TRANSFERÊNCIA DE LOCALIZAÇÃO

11.1 Mudanças de localização devem ser comunicadas à LOCADORA com 10 dias de antecedência.

CLÁUSULA DÉCIMA SEGUNDA – RESCISÃO DO CONTRATO

12.1 Rescisão requer aviso prévio de 30 dias por escrito.

12.2 Descumprimento pode resultar em pagamento equivalente a uma mensalidade.

12.3 Descumprimento grave permite rescisão imediata sem aviso prévio.

12.4 Liquidação de obrigações deve ocorrer dentro de 20 dias após rescisão.

CLÁUSULA DÉCIMA TERCEIRA – MULTA CONTRATUAL

13.1 Rescisão antecipada implica multa de 30% sobre uma mensalidade.

CLÁUSULA DÉCIMA QUARTA – CONFIDENCIALIDADE

14.1 Informações técnicas e comerciais são confidenciais entre as partes.

14.2 O descumprimento gera responsabilidade por perdas e danos.

CLÁUSULA DÉCIMA QUINTA – DISPOSIÇÕES GERAIS

15.1 Contrato sem vícios de vontade, com representação legal adequada.

15.2 Cláusulas inválidas não invalidam o contrato; demais cláusulas permanecem válidas.

15.3 Contrato irrevogável e irretratável; alterações requerem aditivo por escrito.

15.4 Contrato é título executivo extrajudicial.
  `;
};


// Documento PDF com o contrato gerado
const MeuDocumento = (params) => {
  const contratoTexto = gerarTextoContratoP1(params);
  const contratoTexto2 = gerarTextoContratoP2(params);
  
  return (
    <Document>
      <Page style={styles.page}>
        <Text style={styles.title}>CONTRATO DE LOCAÇÃO DE BENS MÓVEIS</Text>
        <View style={styles.section}>
          {contratoTexto.split('\n').map((line, index) => (
            <Text key={index}>{line.trim()}</Text>
          ))}
        </View>
        {/* Exemplo de uma cláusula específica */}

        
        {/* Tabela */}
      <View style={styles.table}>
        <View style={styles.tableRow}>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>ITEM</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>QUANT.</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>EQUIPAMENTO</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>NÚMEROS DE SÉRIE</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>LOCAÇÃO</Text>
          </View>
        </View>

        <View style={styles.tableRow}>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>1</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>2</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>Máquina a Fio Diamantado 75C</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>MF75S20046, MF75D20022</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>R$ 30.000,00</Text>
          </View>
        </View>
      </View>

      <View style={styles.section}>
          {contratoTexto2.split('\n').map((line, index) => (
            <Text key={index}>{line.trim()}</Text>
          ))}
        </View>
        
      </Page>
    </Document>
  );
};

// Componente para exibir o PDF na tela
const CPDF = () => {
  const { state } = useLocation();

  console.log("Dados preenchidos:", state);

  return (
    <PDFViewer width="100%" height="600px">
      <MeuDocumento
      //Locadora
        nomeLocadora={state?.nomelocadora || ""}
        cnpjLocadora={state?.cnpjLocadora || ""}
      //Locadora | Banco
        numeroConta={state?.numeroConta || ""}
        numeroAgencia={state?.numeroAgencia || ""}
      //Locadora | Endereço
        cep={state?.cep || ""}
        cidade={state?.cidade || ""}
        logradouro={state?.logradouro || ""}
        numeroendereco={state?.numeroendereco || ""}
        complemento={state?.complemento || ""}
        uf={state?.uf || ""}
      //Locadora | Socio
        nomeAdmLocadora={state?.nomeAdmLocadora || ""}
        cpf={state?.cpf || ""}
        orgaoemissor={state?.orgaoemissor || ""}
        nacionalidade={state?.nacionalidade || ""}
        estadocivil={state?.estadocivil || ""}
      //Locadora | Socio | Endereço
        logradouroadm = {state?.logradouroadm || ""}
        cepadm = {state?.cepadm || ""}
        ufadm = {state?.ufadm || ""}
        complementoadm = {state?.complementoadm || ""}
        cidadeadm = {state?.cidadeadm || ""}
        numeroenderecoadm = {state?.numeroenderecoadm || ""}
      />
    </PDFViewer>
  );
};

export default CPDF;
