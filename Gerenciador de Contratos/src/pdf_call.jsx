import React from 'react';
import { Page, Text, View, Document, StyleSheet, PDFViewer } from '@react-pdf/renderer';

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

// Documento PDF com tabela
const MeuDocumento = () => (
  <Document>
    <Page style={styles.page}>
      <Text style={styles.title}>Relatório</Text>
      <View style={styles.section}>
        <Text>Nome: João da Silva</Text>
        <Text>Email: joao@email.com</Text>
      </View>

      {/* Tabela */}
      <View style={styles.table}>
        <View style={styles.tableRow}>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>Produto</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>Quantidade</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>Preço Unitário</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>Total</Text>
          </View>
        </View>

        <View style={styles.tableRow}>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>Computador</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>2</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>R$ 2500</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>R$ 5000</Text>
          </View>
        </View>
      </View>

      <View style={styles.section}>
        <Text>Dados de Venda:</Text>
        <Text>Produto: Computador</Text>
        <Text>Valor: R$ 2500</Text>
      </View>
    </Page>
  </Document>
);

// Componente para exibir o PDF na tela
const CPDF = () => (
  <PDFViewer width="100%" height="600px">
    <MeuDocumento />
  </PDFViewer>
);

export default CPDF;
