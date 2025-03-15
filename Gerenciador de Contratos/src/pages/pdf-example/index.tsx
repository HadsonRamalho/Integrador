import Layout from "@/layouts/default";
import "@/components/machine/machine.css";

import {
  Document,
  Page,
  Text,
  View,
  StyleSheet,

  
} from "@react-pdf/renderer";
import {
  Card,
  CardFooter,
  CardHeader,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { ContractPDF } from "@/interfaces/contract";
import { formatCurrency, formatDate } from "@/services/api/format/format";
import { Machine } from "@/interfaces/machine";

// react-pdf/renderer

export const PdfDocument = ({ contract }: { contract: ContractPDF }) => {
  const dataContrato = formatDate(contract.datacontrato);
  const styles = StyleSheet.create({
    page: {
      flexDirection: "column",
      backgroundColor: "#fff",
      padding: 25,
      fontFamily: 'Times-Roman',

    },
    section: {
      backgroundColor: "#fff",
      borderRadius: 15,
      fontFamily: 'Times-Roman',
    },
    title: {
      marginBottom: 15,
      fontSize: 16,
      fontFamily: 'Times-Roman',
      textAlign: "center",
    },
    mainTitle: {
      marginBottom: 15,
      fontSize: 20,
      fontFamily: 'Times-Bold',
      textAlign: "center",
    },
    paragraph: {
      fontSize: 12,
      textAlign: "justify",
      lineHeight: 1.5,
      fontFamily: 'Times-Roman',
      marginBottom: 10
    },
    bold: {
      fontWeight: 900,
      fontFamily: 'Times-Bold',
    },
    space: {
      marginBottom: 8,
    },

  });

  const tableStyles = StyleSheet.create({
    table: {
      width: '100%',
      borderWidth: 1,
      borderColor: 'black'
    },
    row: {
      display: 'flex',
      flexDirection: 'row',
      borderTop: '1px solid #EEE',
      paddingTop: 8,
      paddingBottom: 8,
    },
    header: {
      borderTop: 'none',
      borderWidth: 1,
      borderColor: 'black'
    },
    bold: {
      fontWeight: 'bold',
    },
    bold2: {
      fontWeight: 900,
      fontFamily: 'Times-Bold',
    },
    col1: {
      width: '33%',
    },
    col2: {
      width: '33%',
    },
    col3: {
      width: '33%',
    },
    col4: {
      width: '33%',
      fontSize: 13
    },
    col5: {
      width: '27%',
    },
  })
  
  const ReportTable = ({ data }: { data: ContractPDF[] }) => {
    return (
      <View style={tableStyles.table}>
        <View style={[tableStyles.row, tableStyles.bold2, tableStyles.header]}>
          <Text style={tableStyles.col1}>Item</Text>
          <Text style={tableStyles.col2}>Número de Série</Text>
          <Text style={tableStyles.col3}>Valor da Locação</Text>
        </View>
        {data.map((row, i) => (
          <View key={i} style={tableStyles.row} wrap={false}>
            <Text style={tableStyles.col4}>
              <Text style={tableStyles.bold}>{row.nomemaquina}</Text>
            </Text>
            <Text style={tableStyles.col4}>{row.numeroseriemaquina}</Text>
            <Text style={tableStyles.col4}>{formatCurrency(row.valorlocacao)}</Text>
          </View>
        ))}
      </View>
    )
  }


  return(
  <Document>
    <Page size="A4" style={styles.page}>
      <View style={styles.section}>
        <Text style={styles.mainTitle}>CONTRATO DE LOCAÇÃO DE BENS MÓVEIS</Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>LOCADORA: {contract.nomelocador}</Text>, inscrita no 
           {" " + contract.tipodocumentolocador + " "} sob o nº <Text style={styles.bold}>
            {" " + contract.documentolocador + " "}</Text>
          , com endereço em <Text style={styles.bold}>Rua {" " + contract.logradouroenderecolocador + " " }, 
            {" " + contract.cidadeenderecolocador}/{contract.estadoenderecolocador},
          N° {" " + contract.numeroenderecolocador+ " " } e com complemento "{contract.complementoenderecolocador}"</Text>. 
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>Locatária: {contract.nomelocatario}</Text>, inscrita no 
           {" " + contract.tipodocumentolocatario + " "} sob o nº <Text style={styles.bold}>
            {" " + contract.documentolocatario + " "}</Text>
          , com endereço em <Text style={styles.bold}>Rua {" " + contract.logradouroenderecolocatario + " " }, 
            {" " + contract.cidadeenderecolocatario}/{contract.estadoenderecolocatario},
          N° {" " + contract.numeroenderecolocatario+ " " } e com complemento "{contract.complementoenderecolocatario}"</Text>. 
        </Text>

        <Text style={styles.paragraph}>
          Ambas as PARTES
          aqui representadas por quem de direito, têm justo e contratado entre
          si a celebração do presente Contrato de Locação de Bens Móveis, que
          reger-se-á de acordo com as cláusulas e condições aqui previstas.
        </Text>
        
        <Text style={styles.title}>
          DEFINIÇÕES 
        </Text>

        <Text style={styles.paragraph}>
          As expressões abaixo, sempre que grafadas neste contrato
          em “caixa alta”, terão para todos os fins e efeitos de direito, os
          seguintes significados: 
        </Text>
        <Text style={styles.paragraph}>
          a) <Text style={styles.bold}>LOCADORA</Text>: Pessoa Jurídica que dará o(s) bem(ns) em locação. 
        </Text>
        <Text style={styles.paragraph}>
        b) <Text style={styles.bold}>LOCATÁRIA</Text>: Pessoa Jurídica que receberá o(s)
        bem(ns) dado(s) em locação. 
        </Text>
        <Text style={styles.paragraph}>
        c) PARTES: São a <Text style={styles.bold}>LOCADORA</Text> e a <Text style={styles.bold}>LOCATÁRIA</Text>
          qualificadas no preâmbulo deste contrato, quando consideradas em
          conjunto. 
        </Text>
        <Text style={styles.paragraph}>
        d) PARTE: São a <Text style={styles.bold}>LOCADORA</Text> e a <Text style={styles.bold}>LOCATÁRIA</Text> qualificadas no
        preâmbulo deste contrato, quando consideradas isoladamente. 
        </Text>
        <Text style={styles.title}>
        CLÁUSULA PRIMEIRA – DO OBJETO, PRAZO E USO
        </Text>
        <Text style={styles.paragraph}>
        <Text style={styles.bold}>1.1</Text> A <Text style={styles.bold}>LOCADORA</Text> dará em locação à <Text style={styles.bold}> LOCATÁRIA</Text>, bens móveis que declara ser a legítima possuidora e/ou
          proprietária. Os bens estão descritos conforme lista de equipamentos
          abaixo. Assim, os objetos desta locação são os seguintes
          equipamentos:
        </Text>

        <ReportTable data={[contract]}/>
        
        <Text style={styles.paragraph}></Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>1.1.1 </Text> Os equipamentos
          previstos nessa cláusula, sendo objetos do presente contrato, serão
          retirados no seguinte endereço:          
        </Text>
        <Text style={[styles.paragraph, styles.bold]}>
          • {contract.cidadeenderecoretirada}, {contract.estadoenderecoretirada},  {contract.cependerecoretirada}
          , {contract.bairroenderecoretirada}, {contract.logradouroenderecoretirada}, N° {contract.numeroenderecoretirada}, 
           com complemento "{contract.complementoenderecoretirada}".
        </Text>
        
        <Text style={styles.paragraph}>
        <Text style={styles.bold}>1.1.2 </Text>
         Os equipamentos serão utilizados pela <Text style={styles.bold}>LOCATÁRIA</Text> no endereço
          supramencionado, sendo que todas as responsabilidades sobre o uso do
          equipamento permanecem integrais à <Text style={styles.bold}>LOCATÁRIA</Text>, referentes à guarda,
          ao depósito e ao uso dos bens.
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>1.1.3 </Text> Os equipamentos permanecerão sob a guarda,
          depósito e responsabilidade da <Text style={styles.bold}>LOCATÁRIA</Text> na qualidade de fiel
          depositária, declarando assumir todas as responsabilidades e
          encargos que lhe incumbem de acordo com a lei civil e penal.          
        </Text>

        <Text style={styles.title}>
          CLÁUSULA SEGUNDA – PRAZO DE VIGÊNCIA DO CONTRATO 
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>2.1 </Text> O prazo da
          presente locação é de {contract.prazolocacao} {contract.medidatempolocacao}, passando a vigorar a partir
          da data de entrega dos equipamentos, nos termos previstos da Cláusula Primeira, item 1.1.1. 
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>2.1.1 </Text> Findo prazo acima estipulado, os equipamentos 
          deverão ser devolvidos à <Text style={styles.bold}>LOCADORA</Text> nas mesmas condições recebidas,
          qual seja, conforme o laudo de entrega inicial e no mesmo local em que se encontrava 
          quando locado.  
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}> 2.1.2 </Text>Os bens locados apenas
          poderão ser operados por Operador devidamente habilitado para
          manuseio de cada equipamento. 
        </Text>

        <Text style={styles.title}>CLAÚSULA TERCEIRA – DO VALOR E FORMA DE PAGAMENTO </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>3.1</Text> A <Text style={styles.bold}> LOCATÁRIA </Text> 
          pagará à <Text style={styles.bold}> LOCADORA </Text> a título de
          contraprestação pela locação dos equipamentos, o valor de {formatCurrency(contract.valorlocacao)}.
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}> 3.1.1 </Text>  O pagamento do valor relativo à locação será efetuado imediatamente, mediante transferência
          bancária para a LOCADORA, na conta que segue:
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>Nome do Banco: </Text> {contract.nomebanco}
        </Text>
        <Text style={styles.paragraph}>
          <Text style={styles.bold}>Número da Conta: </Text> {contract.numerocontabanco}
        </Text>
        <Text style={styles.paragraph}>
          <Text style={styles.bold}>Número da Agência: </Text> {contract.numeroagenciabanco}
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}> 3.1.2 </Text> 
          O atraso no pagamento do acordo da cláusula acima sujeita a 
          <Text style={styles.bold}> LOCATÁRIA </Text>
          ao pagamento de multa de 2% (dois por cento) e juros de 1% (um por
          cento) ao mês, sobre o valor do débito devidamente corrigido pelo
          valor positivo do índice IPCA ou por outro que venha eventualmente a
          substitui-lo, sem prejuízo da rescisão da locação que poderá ser
          exigida pela <Text style={styles.bold}> LOCADORA </Text>, após notificação à <Text style={styles.bold}> LOCATÁRIA </Text> oportunizando o
          adimplemento do débito mais encargos no prazo de 10 (dez) dias.
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}> 3.1.3 </Text> Em caso de atraso no pagamento, a <Text style={styles.bold}> LOCADORA </Text> poderá
          incluir a <Text style={styles.bold}> LOCATÁRIA </Text> na lista de inadimplentes do SPC e/ou outras
          instituições de proteção ao crédito. 
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>  3.1.4 </Text>Na formação de preço da
          locação levou-se em conta que caberá à <Text style={styles.bold}> LOCADORA </Text> o custeio de todas
          as manutenções preventivas do equipamento, devendo tais manutenções serem feitas conforme
          especificações técnicas dos fabricantes dos equipamentos. 
        </Text>
          
        <Text style={styles.title}>
        CLAÚSULA QUARTA – DO ESTADO DE CONSERVAÇÃO DOS EQUIPAMENTOS
        </Text>
  
        <Text style={styles.paragraph}>
        <Text style={styles.bold}> 4.1 </Text> Os equipamentos, objetos da presente locação encontram-se no estado de conservação e
          funcionamento constantes dos laudos de entrega/recebimento inicial,
          fato que ambas as PARTES expressamente reconhecem.
        </Text>


        <Text style={styles.title}>
        CLAÚSULA QUINTA – DA UTILIZAÇÃO E MANUTENÇÃO DOS BENS, INSPEÇÃO, IDENTIFICAÇÃO, SIGILO E PATENTE
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}> 5.1 </Text> A <Text style={styles.bold}> LOCATÁRIA </Text> se obriga a utilizar os bens única e
          exclusivamente para as atividades a que se destinam e a operá-los de
          acordo com as recomendações técnicas do fabricante, por pessoa
          tecnicamente qualificada a fazê-lo. 
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}> 5.2 </Text> À <Text style={styles.bold}> LOCATÁRIA </Text> são transferidas
          todas as garantias e o direito de assistência técnica, dados pelo
          fabricante, ficando a <Text style={styles.bold}> LOCADORA </Text>, responsável pela execução e/ou
          custeio das manutenções preventivas e por quaisquer outras
          decorrentes do uso normal dos equipamentos. Mas, ressalta-se que a
          LOCADORA poderá ser acionada sempre que for necessário. 
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>5.3 </Text> A <Text style={styles.bold}> LOCATÁRIA </Text> se
         responsabiliza pelos danos que os bens venham a sofrer
          na constância do contrato, em virtude de mau uso ou negligência na
          sua conservação, competindo-lhe tomar todas as providências para a
          manutenção, serviços e reparos necessários. São de responsabilidade
          da <Text style={styles.bold}> LOCADORA</Text>, já estando contemplados
          no preço da locação, os danos e depreciação decorrentes do uso normal dos equipamentos e/ou quando
          decorrentes de caso fortuito ou força maior.
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>5.4 </Text> A <Text style={styles.bold}> LOCADORA </Text> 
          responderá pelos vícios ou defeitos do bem que sejam anteriores à locação, 
          realizando o reparo ou substituição de peças, quando referidos vícios forem 
          comprovadamente resultantes de fabricação ou imperfeição da mão de obra utilizada.
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>5.5 </Text> Se qualquer peça vier a se destruir, estragar ou a se inutilizar
          por negligência ou mau uso, de qualquer forma, na constância do
          contrato, a <Text style={styles.bold}> LOCATÁRIA </Text> se obriga, com o expresso consentimento da
          <Text style={styles.bold}> LOCADORA </Text>, a substituí-la, utilizando-se das peças adequadas e
          especificadas a tal substituição. 
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>5.6 </Text>A <Text style={styles.bold}> LOCATÁRIA </Text> não
          poderá adaptar ou instalar quaisquer peças ou acessórios aos bens,
          que alterem as condições técnicas e normais de uso. 
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>5.7 </Text>Na
          ocorrência de não funcionamento dos bens, por qualquer motivo, a
          <Text style={styles.bold}> LOCATÁRIA </Text> não poderá pleitear diminuição, suspensão ou cessação de
          pagamento das obrigações pecuniárias ou indenização por PARTE da
          <Text style={styles.bold}> LOCADORA</Text>, cabendo a <Text style={styles.bold}> LOCADORA</Text>, providenciar o funcionamento desses,
          substituindo-os se for o caso. A <Text style={styles.bold}> LOCADORA </Text> compromete-se a
          transferir à <Text style={styles.bold}> LOCATÁRIA </Text> todos os direitos e garantias dos bens. 
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>5.8 </Text> A
        <Text style={styles.bold}> LOCADORA </Text>, a qualquer tempo, poderá inspecionar os bens e exigir que
          sejam tomadas providências para preservação e bom funcionamento dos
          mesmos, sem que isso implique em transferência para ela das
          responsabilidades da <Text style={styles.bold}> LOCATÁRIA</Text>. 
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>5.9 </Text>A <Text style={styles.bold}> LOCATÁRIA </Text> por si e por seus
          empregados ou prepostos, fica obrigada a manter sigilo sobre todas
          as informações confidenciais e/ou protegidas por registro de
          patente, que lhe venham a ser transmitidas pelo fabricante ou
          fornecedor, responsabilizando-se por qualquer prejuízo, reclamação
          ou pleito oriundo de eventual violação. 
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>5.10 </Text>Além das condições aqui
          especificadas, a <Text style={styles.bold}> LOCATÁRIA </Text> obriga-se a comunicar prontamente por
          escrito, qualquer deslocamento dos bens. 
        </Text>

        <Text style={styles.title}>
        CLAÚSULA SEXTA – DA
        RESPONSABILIDADE PELA UTILIZAÇÃO DOS EQUIPAMENTOS LOCADOS
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>6.1 </Text>
          Quaisquer acidentes ocorridos com os equipamentos locados ou por ele
          causados à terceiros, desde sua entrega até sua efetiva devolução e
          recebimento pela <Text style={styles.bold}> LOCADORA </Text>, serão de exclusiva responsabilidade da
          <Text style={styles.bold}> LOCATÁRIA</Text>, ficando a <Text style={styles.bold}> LOCADORA </Text> excluída de quaisquer
          responsabilidades civis, criminais, trabalhistas ou outras, salvo se
          restar comprovada a culpa direta ou indireta da <Text style={styles.bold}> LOCADORA </Text> para
          ocorrência do evento, caso em que responderão conforme o seu grau
          individual de responsabilidade. 
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>6.2 </Text>Caso a <Text style={styles.bold}> LOCADORA </Text> constate a
          utilização dos equipamentos em condições impróprias, conflitantes
          com as Normas de Segurança do Trabalho do Ministério do Trabalho e
          Previdência Social ou recomendações da Associação Brasileira de
          Normas Técnicas, que venham oferecer risco a integridade física dos
          funcionários e/ou de terceiros ou que ameacem causar danos ao
          equipamento, poderá considerar rescindido o presente contrato. 
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>6.3 </Text>
          No caso de ocorrência de quaisquer eventos envolvendo responsabilidade por danos corporais
          e/ou materiais e/ou danos morais causados a terceiros e decorrentes direta ou
          indiretamente da propriedade, uso, transporte ou operação dos bens, caberá única e
          exclusivamente à <Text style={styles.bold}> LOCATÁRIA</Text>, independentemente da 
          existência ou não de seguro, a responsabilidade decorrente de tais eventos, inclusive
          pelas despesas extrajudiciais e judiciais, honorários advocatícios, constituição de
          capital cuja renda assegure o cabal cumprimento de indenização, e tudo o mais que 
          for necessário para que a <Text style={styles.bold}> LOCADORA </Text> não sofra 
          qualquer gravame ou prejuízo, salvo se restar comprovado que o evento se deu por
          falta de informação necessária à <Text style={styles.bold}> LOCATÁRIA </Text> por
          parte da <Text style={styles.bold}> LOCADORA</Text>, caso em que responderão conforme o 
          seu grau individual de responsabilidade.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>6.4 </Text>
          Independentemente e sem prejuízo de sua responsabilidade, a 
          <Text style={styles.bold}> LOCATÁRIA </Text> se obriga a dar imediato conhecimento por
          escrito à <Text style={styles.bold}> LOCADORA </Text> de qualquer reclamação, citação,
          intimação, carta ou documento que receber, com relação a qualquer ocorrência envolvendo 
          danos corporais e/ou materiais e/ou pecuniários e/ou danos morais, relacionados com os bens.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>6.5 </Text>
          As obrigações legais perante o Poder Público, concernentes à lavra e aos cuidados ambientais,
          serão de competência e responsabilidade exclusiva da <Text style={styles.bold}> LOCATÁRIA</Text>, 
          que assume o compromisso de diligenciar, às suas expensas, as autorizações pertinentes à 
          atividade a ser desempenhada e responderá pela não observância das imposições normativas 
          inerentes à espécie, bem como por todo e qualquer dano ambiental eventualmente causado.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>6.6 </Text>
          Cabe à <Text style={styles.bold}>LOCATÁRIA</Text>, única e exclusivamente, contratar 
          pessoal técnico especializado para fim de operar os equipamentos, cumprindo todas as 
          obrigações legais e exigências ambientais para manutenção da operação.
        </Text>

        <Text style={styles.title}>
          CLAÚSULA SÉTIMA – DA DEVOLUÇÃO DOS EQUIPAMENTOS
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>7.1 </Text>
          Finda a locação, a <Text style={styles.bold}> LOCATÁRIA </Text> devolverá os equipamentos 
          locados descritos na Cláusula Primeira, arcando com os devidos custos, no endereço da 
          <Text style={styles.bold}> LOCADORA </Text> localizada na cidade {contract.cidadeenderecolocador}, no estado {contract.estadoenderecolocador}.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>7.2 </Text>
          Na ocasião da devolução, os equipamentos serão vistoriados pela <Text style={styles.bold}> LOCADORA </Text> e essa, no ato da devolução, fará os respectivos relatórios das quantidades e condições dos equipamentos.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>7.3 </Text>
          Qualquer irregularidade, peça faltante, quebra ou desgaste dos equipamentos, exceto os 
          desgastes naturais de uso, será apontada no relatório referido no parágrafo anterior, 
          que deverá ser assinado pela pessoa que fez acompanhar a devolução dos equipamentos, 
          considerando-se essa como preposta da <Text style={styles.bold}> LOCATÁRIA </Text>, o que 
          desde já deixa reconhecido.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>7.4 </Text>
          As avarias verificadas que não se enquadrem em depreciação por uso normal dos equipamentos
          locados, em atividade comum para seu uso, desde que já não apontadas no laudo de recebimento 
          dos equipamentos feito no início do contrato, constatadas de acordo com o parágrafo 
          anterior, serão reparadas pela <Text style={styles.bold}> LOCATÁRIA </Text>, sendo o 
          presente contrato apenas considerado encerrado para todos os fins após a realização dos 
          devidos reparos. 

          Para que não haja dúvida, não serão objeto de indenização ou reparação à 
          <Text style={styles.bold}> LOCADORA </Text> quaisquer reparos que sejam decorrentes de 
          depreciação ou desgaste por uso normal dos equipamentos locados.
        </Text>

        <Text style={styles.title}>
          CLAÚSULA OITAVA – DO EXTRAVIO E DANIFICAÇÃO TOTAL DOS EQUIPAMENTOS
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>8.1 </Text>
          Havendo danificação total do equipamento locado, por culpa da 
          <Text style={styles.bold}> LOCATÁRIA </Text>, essa se responsabilizará pelo seu pagamento, 
          cujo preço será o de mercado em vigor na data da constatação para equipamentos do mesmo ano 
          de fabricação, horas semelhantes de utilização e mesmo tipo de uso dos equipamentos locados, 
          podendo a <Text style={styles.bold}> LOCATÁRIA </Text> optar por pagar o referido valor ou 
          adquirir outros equipamentos semelhantes aos locados e restituí-los à 
          <Text style={styles.bold}> LOCADORA </Text>.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>8.2 </Text>
          A <Text style={styles.bold}> LOCATÁRIA </Text> se responsabiliza pelo equipamento 
          em caso de extravio durante o período de vigência deste contrato, salvo se restar 
          comprovado que não poderia tê-lo evitado, hipótese em que a 
          <Text style={styles.bold}> LOCADORA </Text> sofrerá a perda e a obrigação se resolverá.
        </Text>

        <Text style={styles.title}>
          CLAÚSULA NONA – DA TRIBUTAÇÃO
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>9.1 </Text>
          Todos os tributos incidentes sobre a locação objeto deste Contrato serão arcados e 
          devidos pela <Text style={styles.bold}> LOCADORA </Text>, devendo a 
          <Text style={styles.bold}> LOCATÁRIA </Text> restar isenta de qualquer acréscimo ou 
          pagamento de tributos relacionados à locação. Ademais, o valor do aluguel decorrente do 
          presente contrato não estará sujeito à retenção do imposto por 
          <Text style={styles.bold}> PARTE </Text> da <Text style={styles.bold}> LOCATÁRIA </Text>, 
          o que não afastará a obrigação da <Text style={styles.bold}> LOCADORA </Text> de pagar 
          pontualmente todos os tributos devidos.
        </Text>

        <Text style={styles.title}>
          CLAÚSULA DÉCIMA – DA POSSIBILIDADE DE SUB-LOCAÇÃO, EMPRÉSTIMO OU ARRENDAMENTO DOS EQUIPAMENTOS LOCADOS
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>10.1 </Text>
          A <Text style={styles.bold}> LOCATÁRIA </Text> poderá, mediante prévia comunicação e 
          expressa autorização da <Text style={styles.bold}> LOCADORA </Text>, sublocar, emprestar 
          ou arrendar os equipamentos constantes do presente contrato para outra empresa do seu 
          grupo econômico, desde que mantidas e observadas todas as obrigações previstas neste 
          Contrato. A previsão desta Cláusula não abrange, por qualquer forma, a cessão, sublocação 
          ou empréstimo dos equipamentos para terceiros não integrantes do grupo econômico da 
          <Text style={styles.bold}> LOCATÁRIA </Text>, o que resta <Text style={styles.bold}> 
          EXPRESSAMENTE PROIBIDO </Text>.
        </Text>

        <Text style={styles.title}>
          CLAÚSULA DÉCIMA PRIMEIRA – TRANSFERÊNCIA DE LOCALIZAÇÃO DOS EQUIPAMENTOS
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>11.1 </Text>
          As transferências de localização dos equipamentos deverão ser comunicadas e acordadas com a 
          <Text style={styles.bold}>LOCADORA</Text> no prazo de até 3 (três) dias antes da data da mudança.
        </Text>

        <Text style={styles.title}>
          CLÁUSULA DÉCIMA SEGUNDA – MULTA CONTRATUAL
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>12.1 </Text>
          No caso de rescisão antecipada por qualquer das <Text style={styles.bold}>PARTES</Text>, será aplicada multa de 30% (trinta por cento) sobre a mensalidade de locação.
        </Text>

        <Text style={styles.title}>
          CLÁUSULA DÉCIMA TERCEIRA – DA CONFIDENCIALIDADE
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>13.1 </Text>
          As <Text style={styles.bold}>PARTES</Text> contratantes, por força do presente 
          instrumento, se comprometem a manter em sigilo todas as informações técnicas e 
          comerciais, relativas à contratação ora realizada.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>13.2 </Text>
          O descumprimento da obrigação de sigilo e de confidencialidade poderá acarretar:
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>13.2.1 </Text>
          Na responsabilidade por perdas e danos da <Text style={styles.bold}>PARTE</Text> 
          infratora à <Text style={styles.bold}>PARTE</Text> inocente.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>13.2.2 </Text>
          Adoção dos remédios jurídicos e sanções cabíveis e demais legislações pertinentes.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>13.2.3 </Text>
          Só será legítima, como motivo de exceção à obrigatoriedade de sigilo, a ocorrência nas 
          seguintes hipóteses:
        </Text>

        <Text style={styles.paragraph}>
          a) A informação já era conhecida pela <Text style={styles.bold}>PARTE</Text> receptora 
          anteriormente às tratativas do negócio jurídico, sem quebra de confidencialidade.
        </Text>
        <Text style={styles.paragraph}>
          b) Houve prévia e expressa anuência da <Text style={styles.bold}>PARTE</Text> divulgadora 
          quanto à liberação da obrigação de sigilo e confidencialidade.
        </Text>
        <Text style={styles.paragraph}>
          c) A informação foi comprovadamente obtida por outra fonte, de forma legal e legítima, 
          independentemente deste contrato.
        </Text>
        <Text style={styles.paragraph}>
          d) Determinação judicial ou governamental para conhecimento das informações, desde que 
          notificadas imediatamente à <Text style={styles.bold}>PARTE</Text> divulgadora, 
          previamente à sua liberação, sendo requerido segredo de justiça no seu trato judicial 
          ou administrativo.
        </Text>

        <Text style={styles.title}>
          CLÁUSULA DÉCIMA QUARTA – DISPOSIÇÕES GERAIS
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>14.1 </Text>
          As <Text style={styles.bold}>PARTES</Text> declaram que a presente contratação se 
          faz livre de defeitos que viciem a manifestação de vontade, que se ultima por intermédio 
          de representantes legais com poderes suficientes para obrigá-las e que foi submetida às 
          suas assessorias jurídicas, que concordaram quanto ao atendimento da vontade das partes, 
          licitude ou não proscrição do objeto.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>14.2 </Text>
          Caso qualquer das cláusulas deste instrumento venha a ser declarada nula, ilegal, 
          inválida ou inexequível, nos termos da legislação brasileira, as demais cláusulas 
          continuarão em pleno vigor.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>14.3 </Text>
          O presente instrumento é celebrado em caráter irrevogável e irretratável, obrigando as 
          <Text style={styles.bold}>PARTES</Text>, seus herdeiros e sucessores.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>14.4 </Text>
          Qualquer modificação do presente instrumento será realizada unicamente por meio de 
          Termo Aditivo escrito, concluído e assinado por ambas as 
          <Text style={styles.bold}>PARTES</Text>.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>14.5 </Text>
          Este contrato constitui o acordo completo e final entre as 
          <Text style={styles.bold}>PARTES</Text>, substituindo todos os entendimentos anteriores.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>14.6 </Text>
          As comunicações entre as <Text style={styles.bold}>PARTES</Text> serão sempre 
          feitas por escrito ou por e-mail.
        </Text>

        <Text style={styles.paragraph}>
          <Text style={styles.bold}>14.7 </Text>
          O término do prazo contratual não afeta a responsabilidade das 
          <Text style={styles.bold}>PARTES</Text> no que tange ao sigilo, obrigações trabalhistas, 
          fiscais e civis.
        </Text>



        <Text style={styles.paragraph}>
        <Text style={styles.bold}>14.8 </Text> A eventual aceitação, por uma das PARTES, da
          inexecução, pela outra, de qualquer das condições aqui
          estabelecidas, a qualquer tempo, não constituirá novação, devendo
          ser interpretada como mera liberalidade, não implicando, portanto,
          na desistência de exigir o cumprimento das disposições aqui contidas
          ou do direito de requerer futuramente a total execução de cada uma
          das obrigações estabelecidas neste contrato, bem como de pleitear
          perdas e danos. 
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>14.9 </Text>As comunicações entre as PARTES contratantes,
          relacionadas com o acompanhamento e controle do presente contrato,
          serão sempre feitas por escrito ou por e-mail. 
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>15.10 </Text>O término do
          prazo contratual, a resolução, a resilição ou a rescisão deste
          contrato não afeta a responsabilidade das PARTES no que tange ao
          sigilo a ser observado em face deste contrato, bem como eventuais
          ressarcimentos relativos às obrigações de cunho trabalhista, fiscal,
          cível ou previdenciária, ou ainda dos danos causados a terceiros
          decorrentes de culpa ou dolo da outra PARTE, seus empregados,
          prepostos e/ou sócios. 
        </Text>

        <Text style={styles.title}>
          CLÁUSULA DÉCIMA SEXTA – DA ELEIÇÃO DO FORO
        </Text>

        <Text style={styles.paragraph}>
        <Text style={styles.bold}>16.1 </Text>Fica eleito o foro da cidade de
         {contract.cidadeenderecolocador}, {contract.estadoenderecolocador}, para dirimir
          quaisquer divergências provenientes do presente contrato, com
          renúncia a quaisquer outros por mais privilegiados que sejam. E por
          estarem assim justos e contratados, assinam o presente contrato em
          02 (duas) vias de igual teor para o mesmo efeito, juntamente com as
          duas testemunhas abaixo, declarando ser perfeitamente conhecedores
          das condições e termos do presente, aceitando-os na forma como se
          encontram redigidos..
        </Text>

        <Text style={styles.paragraph}>
        {contract.cidadeenderecolocador}, {contract.estadoenderecolocador}, 
        {dataContrato}
        </Text>

        <Text style={styles.paragraph}>
          LOCADORA: _______________________________________
        </Text>

        <Text style={styles.paragraph}>
          LOCATÁRIA: _______________________________________
        </Text>

        <Text>

        </Text>
      </View>
    </Page>
  </Document>)
};

export default function PdfExample() {

  return (
    <Layout>
      <main className="mt-10">
        <div className="mt-40 mb-40">
          <Card>
            <CardHeader>Documento preparado</CardHeader>
            <CardFooter>
              <Button className="ml-10">
                Baixar PDF
              </Button>
            </CardFooter>
          </Card>
        </div>
      </main>
    </Layout>
  );
}
