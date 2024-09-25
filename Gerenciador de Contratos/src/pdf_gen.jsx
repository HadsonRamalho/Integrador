import React from 'react';
import { Page, Text, View, Document, StyleSheet, PDFViewer } from '@react-pdf/renderer';
import { useLocation } from 'react-router-dom';
import { useNavigate } from "react-router-dom";

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

  nomelocatario,
  cnpjlocatario,

  logradourolocatario,
  cidadelocatario,
  uflocatario,

  nomesociolocatario,
  nacionalidadesociolocatario,
  estadocivilsociolocatario,
  cpfsociolocatario,
  logradourosociolocatario,
  cidadesociolocatairo,
  ufsociolocatario

}) => {
  return `
    LOCADORA: ${nomeLocadora}, inscrita no CNPJ sob o nº ${cnpjLocadora}, com sede em ${logradouro}, N° ${numeroendereco} ${complemento},  ${cidade}/${uf}, neste ato representada pelo seu sócio administrador, ${nomeAdmLocadora}, ${nacionalidade}, ${estadocivil}, inscrito no CPF sob nº ${cpf} emitido por ${orgaoemissor}, com endereço em ${cepadm}, ${logradouroadm}, ${numeroenderecoadm}, ${complementoadm}, ${cidadeadm}/${ufadm}.
LOCATÁRIA: ${nomelocatario}, inscrita no CNPJ sob o nº ${cnpjlocatario}, com sede na ${logradourolocatario}, ${cidadelocatario}/${uflocatario}, representada por ${nomesociolocatario}, ${nacionalidadesociolocatario}, ${estadocivilsociolocatario}, CPF ${cpfsociolocatario}, residente na ${logradourosociolocatario}, ${cidadesociolocatairo}/${ufsociolocatario}.
Ambas as PARTES aqui representadas por quem de direito, têm justo e contratado entre si a celebração do presente Contrato de Locação de Bens Móveis, que reger-se-á de acordo com as cláusulas e condições aqui previstas.
DEFINIÇÕES


As expressões abaixo, sempre que grafadas neste contrato em “caixa alta”, terão para todos os fins e efeitos de direito, os seguintes significados:

    a) LOCADORA: Pessoa Jurídica que dará o(s) bem(ns) em locação.

    b) LOCATÁRIA: Pessoa Jurídica que receberá o(s) bem(ns) dado(s) em locação.

    c) PARTES: São a LOCADORA e a LOCATÁRIA qualificadas no preâmbulo deste contrato, quando consideradas em conjunto.

    d) PARTE: São a LOCADORA e a LOCATÁRIA qualificadas no preâmbulo deste contrato, quando consideradas isoladamente.

CLÁUSULA PRIMEIRA – DO OBJETO, PRAZO E USO


        1.1 A LOCADORA dará em locação à LOCATÁRIA, bens móveis que declara ser a legítima possuidora e/ou proprietária. Os bens estão descritos conforme lista de equipamentos abaixo. Assim, os objetos desta locação são os seguintes equipamentos:
 `;
};

const gerarTextoContratoP2 = ({logradouro,
  numeroendereco,
  complemento,
  cidade,
  uf,

}) => {
  return `
    1.1.1 Os equipamentos previstos nessa cláusula, sendo objetos do presente contrato, serão retirados no seguinte endereço:

                • ${logradouro}, N° ${numeroendereco} ${complemento},  ${cidade}/${uf}

            1.1.2 Os equipamentos serão utilizados pela LOCATÁRIA no endereço supramencionado, sendo que todas as responsabilidades sobre o uso do equipamento permanecem integrais à LOCATÁRIA, referentes à guarda, ao depósito e ao uso dos bens. Mediante prévia comunicação da LOCATÁRIA à LOCADORA, bem como mediante expressa autorização dessa última, os equipamentos locados poderão ser transferidos para outras pedreiras do grupo econômico da LOCATÁRIA, localizadas no Estado de Minas Gerais, mantidas inalteradas todas as obrigações previstas neste Contrato.

            1.1.3 Os equipamentos permanecerão sob a guarda, depósito e responsabilidade da LOCATÁRIA na qualidade de fiel depositária, declarando assumir todas as responsabilidades e encargos que lhe incumbem de acordo com a lei civil e penal.


CLÁUSULA SEGUNDA – PRAZO DE VIGÊNCIA DO CONTRATO


2.1 O prazo da presente locação é de ${prazolocacao} meses, passando a vigorar a partir da data de entrega dos equipamento, qual seja, a data de 23 de novembro de 2023, nos termos previstos da Cláusula Primeira, item 1.1.1.

            2.2.1 Findo prazo acima estipulado, o mesmo poderá ser renovado através de aditivo ou outro instrumento contratual por igual período, com pagamento mínimo do valor previsto conforme Cláusula Terceira, item 3.1, podendo este vir a ser reajustado na ocasião por acordo entre as partes.

            2.2.2 Não havendo interesse em renovação, os equipamentos deverão ser devolvidos à LOCADORA nas mesmas condições recebidas, qual seja, conforme o laudo de entrega inicial e no mesmo local em que se encontrava quando locado.

            2.2.3 Os bens locados somente poderão ser destinados para uso exclusivo ao qual se destinam, qual seja, mineração.

            2.2.4 Os bens locados apenas poderão ser operados por Operador devidamente habilitado para manuseio de cada equipamento.


CLAÚSULA TERCEIRA – DO VALOR E FORMA DE PAGAMENTO


        3.1 A LOCATÁRIA pagará à LOCADORA a título de contraprestação pela locação dos equipamentos, o valor mensal de R$ 19.000,00 (Dezenove Mil Reais) e, na hipótese de renovação deste contrato, os valores passarão a ser corrigidos anualmente pelo valor positivo do índice IPCA ou por outro que venha eventualmente a substituí-lo.

            3.1.1 O pagamento da locação será mensal e adiantado, vencendo todo dia 23 (vinte e três), com início em 23/11/2023, mediante boleto bancário, com exceção da primeira parcela (entrada).

            3.1.2 O pagamento da entrada será efetuado mediante transferência bancária para a LOCADORA, na conta que segue:

DADOS da CONTA


            3.1.3 O atraso no pagamento do acordo da cláusula acima sujeita a LOCATÁRIA ao pagamento de multa de 2% (dois por cento) e juros de 1% (um por cento) ao mês, sobre o valor do débito devidamente corrigido pelo valor positivo do índice IPCA ou por outro que venha eventualmente a substitui-lo, sem prejuízo da rescisão da locação que poderá ser exigida pela LOCADORA, após notificação à LOCATÁRIA oportunizando o adimplemento do débito mais encargos no prazo de 10 (dez) dias.

            3.1.4 Em caso de atraso no pagamento a LOCADORA poderá ainda, incluir a LOCATÁRIA na lista de inadimplentes do SPC e/ou outras instituições de proteção ao crédito.

            3.1.5 Na formação de preço da locação levou-se em conta que caberá à LOCADORA o custeio de todas as manutenções preventivas e daquelas decorrentes do uso normal dos equipamentos, devendo tais manutenções serem feitas conforme especificações técnicas dos fabricantes dos equipamentos.

            3.1.6 Fica devidamente e expressamente ajustado entre as PARTES que o valor mensal acordado neste contrato, conforme Cláusula Terceira, item 3.1, a título de contraprestação pela locação dos equipamentos, está condicionado ao uso de 200 (duzentas) horas por mês. Deste modo, caso esse limite seja ultrapassado, as horas excedentes serão cobradas a parte no fechamento da planilha mensal.


CLAÚSULA QUARTA – DO ESTADO DE CONSERVAÇÃO DOS EQUIPAMENTOS


4.1 Os equipamentos objetos da presente locação encontram-se no estado de conservação e funcionamento constantes dos laudos de entrega/recebimento inicial, fato que ambas as PARTES expressamente reconhecem.


CLAÚSULA QUINTA – DA UTILIZAÇÃO E MANUTENÇÃO DOS BENS, INSPEÇÃO, IDENTIFICAÇÃO, SIGILO E PATENTE


        5.1 A LOCATÁRIA se obriga a utilizar os bens única e exclusivamente para as atividades a que se destinam e a operá-los de acordo com as recomendações técnicas do fabricante, por pessoa tecnicamente qualificada a fazê-lo.

        5.2 À LOCATÁRIA são transferidas todas as garantias e o direito de assistência técnica, dados pelo fabricante, ficando a LOCADORA, responsável pela execução e/ou custeio das manutenções preventivas e por quaisquer outras decorrentes do uso normal dos equipamentos. Mas, ressalta-se que a LOCADORA poderá ser acionada sempre que for necessário.

        5.3 A LOCATÁRIA se responsabiliza pelos danos que os bens venham a sofrer na constância do contrato, em virtude de mau uso ou negligência na sua conservação, competindo-lhe tomar todas as providências para a manutenção, serviços e reparos necessários. São de responsabilidade da LOCADORA, já estando contemplados no preço da locação, os danos e depreciação decorrentes do uso normal dos equipamentos e/ou quando decorrentes de caso fortuito ou força maior. Exclusivamente com relação aos dentes da escavadeira locada, a LOCATÁRIA será responsável por sua manutenção, ainda que decorrente de uso normal do equipamento.

        5.4 A LOCADORA responderá pelos vícios ou defeitos do bem que sejam anteriores à locação, realizando o reparo ou substituição de peças, quando referidos vícios forem comprovadamente resultantes de fabricação ou imperfeição da mão de obra utilizada.

        5.5 Se qualquer peça vier a se destruir, estragar ou a se inutilizar por negligência ou mau uso, de qualquer forma, na constância do contrato, a LOCATÁRIA se obriga, com o expresso consentimento da LOCADORA, a substituí-la, utilizando-se das peças adequadas e especificadas a tal substituição.

        5.6 A LOCADORA ressarcirá a LOCATÁRIA os gastos decorrentes da obrigação retromencionada, salvo se o dano for oriundo de ação ou omissão, única e exclusiva da LOCATÁRIA. Todos os serviços de reparo, manutenção ou substituição de peças somente poderão ser executados por oficiais e/ou pessoal especializado, de indicação do fabricante.

        5.7 A LOCATÁRIA não poderá adaptar ou instalar quaisquer peças ou acessórios aos bens, que alterem as condições técnicas e normais de uso.

        5.8 Na ocorrência de não funcionamento dos bens, por qualquer motivo, a LOCATÁRIA não poderá pleitear diminuição, suspensão ou cessação de pagamento das obrigações pecuniárias ou indenização por PARTE da LOCADORA, cabendo a LOCADORA, providenciar o funcionamento desses, substituindo-os se for o caso. A LOCADORA compromete- se a transferir à LOCATÁRIA todos os direitos e garantias dos bens.

        5.9 A LOCADORA, a qualquer tempo, poderá inspecionar os bens e exigir que sejam tomadas providências para preservação e bom funcionamento dos mesmos, sem que isso implique em transferência para ela das responsabilidades da LOCATÁRIA.

        5.10 A LOCATÁRIA por si e por seus empregados ou prepostos, fica obrigada a manter sigilo sobre todas as informações confidenciais e/ou protegidas por registro de patente, que lhe venham a ser transmitidas pelo fabricante ou fornecedor, responsabilizando-se por qualquer prejuízo, reclamação ou pleito oriundo de eventual violação.

        5.11 Além das condições aqui especificadas, a LOCATÁRIA obriga-se a comunicar prontamente por escrito, qualquer deslocamento dos bens.


CLAÚSULA	SEXTA	–	DA	RESPONSABILIDADE	PELA	UTILIZAÇÃO	DOS EQUIPAMENTOS LOCADOS


        6.1 Quaisquer acidentes ocorridos com os equipamentos locados ou por ele causados à terceiros, desde sua entrega até sua efetiva devolução e recebimento pela LOCADORA, serão de exclusiva responsabilidade da LOCATÁRIA, ficando a LOCADORA excluída de quaisquer responsabilidades civis, criminais, trabalhistas ou outras, salvo se restar comprovada a culpa direta ou indireta da LOCADORA para ocorrência do evento, caso em que responderão conforme o seu grau individual de responsabilidade.

        6.2 Caso a LOCADORA constate a utilização dos equipamentos em condições impróprias, conflitantes com as Normas de Segurança do Trabalho do Ministério do Trabalho e Previdência Social ou recomendações da Associação Brasileira de Normas Técnicas, que venham oferecer risco a integridade física dos funcionários e/ou de terceiros ou que ameacem causar danos ao equipamento, poderá considerar rescindido o presente contrato.

        6.3 No caso de ocorrência de quaisquer eventos envolvendo responsabilidade por danos corporais e/ou materiais e/ou danos morais causados a terceiros e decorrentes direta ou indiretamente da propriedade, uso, transporte ou operação dos bens caberá única e exclusivamente à LOCATÁRIA, independentemente da existência ou não de seguro, a responsabilidade decorrente de tais eventos, inclusive pelas despesas extrajudiciais e judiciais, honorários advocatícios, constituição de capital cuja renda assegure o cabal cumprimento de indenização, e tudo o mais que for necessário para que a LOCADORA não sofra qualquer gravame ou prejuízo, salvo se restar comprovado que o evento se deu por falta de informação necessária à LOCATÁRIA por PARTE da LOCADORA, caso em que responderão conforme o seu grau individual de responsabilidade.

        6.4 Independentemente e sem prejuízo de sua responsabilidade, a LOCATÁRIA se obriga a dar imediato conhecimento por escrito à LOCADORA de qualquer reclamação, citação, intimação, carta ou documento que receber, com relação a qualquer ocorrência envolvendo danos corporais e/ou materiais e/ou pecuniários e/ou danos morais, relacionados com os bens.

        6.5 As obrigações legais perante o Poder Público, concernentes à lavra e aos cuidados ambientais, serão de competência e responsabilidade exclusiva da LOCATÁRIA, que assume o compromisso de diligenciar, às suas expensas, as autorizações pertinentes à atividade a ser desempenhada e responderá pela não observância das imposições normativas inerentes a espécie, bem como por todo e qualquer dano ambiental eventualmente causado.

        6.6 Cabe a LOCATÁRIA, única e exclusivamente, contratar pessoal técnico especializado para fim de operar os equipamentos, cumprindo todas as obrigações legais e exigências ambientais para manutenção da operação.


CLAÚSULA SÉTIMA – DA DEVOLUÇÃO DOS EQUIPAMENTOS


        7.1 Finda a locação, a LOCATÁRIA devolverá os equipamentos locados descritos na Cláusula Primeira, arcando com os devidos custos, no endereço da pedreira da LOCADORA localizada no Distrito de Batatal, Diamantina.

        7.2 Na ocasião da devolução os equipamentos serão vistoriados pela LOCADORA e essa, no ato da devolução, fará os respectivos relatórios das quantidades e condições dos equipamentos.

        7.3 Qualquer irregularidade, peça faltante, quebra ou desgaste dos equipamentos, exceto os desgastes naturais de uso, será apontada no relatório referido no parágrafo anterior, que deverá ser assinado pela pessoa que fez acompanhar a devolução dos equipamentos, considerando-se essa como preposta da LOCATÁRIA, o que desde já deixa reconhecido.

        7.4 As avarias verificadas que não se enquadrem em depreciação por uso normal dos equipamentos locados, em atividade de mineração, desde que já não apontadas no laudo de recebimento dos equipamentos feito no início do contrato, constatadas de acordo com o parágrafo anterior serão reparadas pela LOCATÁRIA, sendo o presente contrato apenas considerado encerrado para todos os fins após a realização dos devidos reparos. Para que não haja dúvida, não serão objeto de indenização ou reparação à LOCADORA quaisquer reparos que sejam decorrentes de depreciação ou desgaste por uso normal, em atividade de mineração, dos equipamentos locados. 


CLAÚSULA	OITAVA	–	DO	EXTRAVIO	E	DANIFICAÇÃO	TOTAL	DOS EQUIPAMENTOS


        8.1 Havendo danificação total do equipamento locado, por culpa da LOCATÁRIA, essa se responsabilizará pelo seu pagamento, cujo preço será o de mercado em vigor na data da constatação para equipamentos do mesmo ano de fabricação, horas semelhantes de utilização e mesmo tipo de uso dos equipamentos locados, podendo a LOCATÁRIA optar por pagar o referido valor ou adquirir outros equipamentos semelhantes aos locados e restituí-los à LOCADORA.

        8.2 A LOCATÁRIA se responsabiliza pelo equipamento em caso de extravio durante o período de vigência deste contrato, salvo se restar comprovado que não poderia tê-lo evitado, hipótese em que a LOCADORA sofrerá a perda e a obrigação se resolverá.


CLAÚSULA NONA – DA TRIBUTAÇÃO


9.1 Todos os tributos incidentes sobre a locação objeto deste Contrato serão arcados e devidos pela LOCADORA, devendo a LOCATÁRIA restar isenta de qualquer acréscimo ou pagamento de tributos relacionados à locação. Ademais, o valor do aluguel decorrente do presente contrato não estará sujeito à retenção do imposto por PARTE da LOCATÁRIA, o que não afastará a obrigação da LOCADORA de pagar pontualmente todos os tributos devidos.


CLAÚSULA DÉCIMA – DA POSSIBILIDADE DE SUB-LOCAÇÃO, EMPRÉSTIMO OU    ARRENDAMENTO DOS EQUIPAMENTOS LOCADOS


10.1 A LOCATÁRIA poderá, mediante prévia comunicação e expressa autorização da LOCADORA, sublocar, emprestar ou arrendar os equipamentos constantes do presente contrato para outra empresa do seu grupo econômico, desde que mantidas e observadas todas as obrigações previstas neste Contrato. A previsão desta Cláusula não abrange, por qualquer forma, a cessão, sublocação ou empréstimo dos equipamentos para terceiros não integrantes do grupo econômico da LOCATÁRIA, o que resta EXPRESSAMENTE PROIBIDO.


CLAÚSULA DÉCIMA PRIMEIRA – TRANSFERÊNCIA DE LOCALIZAÇÃO DOS EQUIPAMENTOS


11.1 As transferências de localização dos equipamentos deverão ser comunicadas e acordadas com a LOCADORA no prazo de até 10 (dez) dias antes da data da mudança.


CLÁUSULA DÉCIMA SEGUNDA – DA RESCISÃO DO CONTRATO


        12.1 O presente contrato poderá ser rescindido, com antecedência de 30 (trinta) dias, por ambas as PARTES, por escrito. Caso a LOCADORA rescinda o contrato previamente ao término do prazo deste Contrato, a LOCADORA deverá arcar exclusivamente com os custos de transporte e remoção dos equipamentos locados.

        12.2 Se o prazo mencionado acima não for respeitado, deverá o denunciante arcar com o valor equivalente a uma mensalidade, listado na Cláusula Terceira, item 3.1.

        12.3 Se houver descumprimento de quaisquer cláusulas deste contrato, a rescisão contratual poderá ocorrer independentemente de notificação prévia.

        12.4 Rescindido o contrato, as PARTES terão o prazo de 20 (vinte) dias corridos para a liquidação de suas obrigações, prazo que poderá ser dilatado, por acordo entre as PARTES, caso seja necessário um período maior para realizar a sua desmobilização.


CLÁUSULA DÉCIMA TERCEIRA – MULTA CONTRATUAL

13.1 No caso de rescisão  antecipada por qualquer das PARTES, será aplicada multa de 30% (trinta por cento), sobre a mensalidade de locação.


CLÁUSULA DÉCIMA QUARTA – DA CONFIDENCIALIDADE


        14.1  As PARTES contratantes, por força do presente instrumento, se comprometem a manter em sigilo todas as informações técnicas e comerciais, relativas à contratação ora realizada.

        14.2  O descumprimento da obrigação de sigilo e de confidencialidade poderá acarretar:

14.2.1 Na responsabilidade por perdas e danos da PARTE infratora à PARTE inocente.

14.2.2 Adoção dos remédios jurídicos e sanções cabíveis e demais legislações pertinentes.

14.2.3 Só será legítima, como motivo de exceção à obrigatoriedade de sigilo, a ocorrência descumprimento nas seguintes hipóteses:

    a) A informação já era conhecida pela PARTE receptora anteriormente as tratativas do negócio jurídico, sem quebra de confidencialidade;

    b) Houve prévia e expressa anuência da PARTE divulgadora quanto à liberação da obrigação de sigilo e confidencialidade;

    c) A informação foi comprovadamente obtida por outra fonte, de forma legal e legítima, independentemente deste contrato;

    d) Determinação judicial ou governamental para conhecimento das informações, desde que notificadas imediatamente a PARTE divulgadora, previamente à sua liberação, sendo requerido segredo de justiça no seu trato judicial ou administrativo.


CLÁUSULA DÉCIMA QUINTA – DISPOSIÇÕES GERAIS


15.1 As PARTES declaram que a presente contratação se faz livre de defeitos que viciem a manifestação de vontade, que se última por intermédio de representantes legais com poderes suficientes para obrigá-las e que foi submetida às suas assessorias jurídicas, que concordaram quanto ao atendimento da vontade das partes, licitude ou não proscrição do objeto.

15.2 Caso qualquer das cláusulas deste instrumento venha a ser declarada nula, ilegal, inválida ou inexequível, nos termos da legislação brasileira, no todo ou em parte, por qualquer razão que seja, as demais cláusulas continuarão em pleno vigor, salvo se por força dessa declaração o presente contrato venha perder o seu objeto ou torná-lo inexequível.

15.3 O presente instrumento é celebrado em caráter irrevogável e irretratável, renunciando as PARTES, expressamente, à faculdade de arrependimento, obrigando as PARTES, seus herdeiros e sucessores a fazerem o presente compromisso em qualquer tempo ou lugar, de forma boa, firme e valiosa.

15.4 Qualquer modificação do presente instrumento será realizada unicamente por meio de Termo Aditivo escrito, concluído e assinado por ambas as PARTES.

15.5 O presente instrumento particular representa título executivo extrajudicial, nos termos do art. 784, inciso III do Código de Processo Civil e está sujeito à execução específica de suas obrigações na forma da lei.

15.6 Este contrato constitui o acordo completo e final entre as PARTES, substituindo todos os entendimentos, compromissos, opções, mensagem via e-mail ou fax, cartas, ou correspondências anteriores e em relação ao assunto objeto deste contrato. Qualquer alteração das obrigações, direitos ou condições, ora pactuados, somente poderá ser procedida mediante a celebração de aditamento contratual escrito.

15.7 Fica expressa e irrevogavelmente estabelecido que a ação ou omissão, bem como abstenção do exercício, por qualquer das PARTES dos direitos ou faculdades que lhe assistem pelo presente contrato, ou a concordância com o atraso no cumprimento das obrigações da outra PARTE, não implicará renúncia daqueles direitos ou faculdades, que poderão ser exercidos, a qualquer tempo, a seu exclusivo critério, salvo se manifestada por escrito. Neste caso, a remissão ou renúncia terá aplicação específica, não significando novação ou renúncia de outros direitos assegurados por lei ou pelo presente contrato.

15.8 A eventual aceitação, por uma das PARTES, da inexecução, pela outra, de qualquer das condições aqui estabelecidas, a qualquer tempo, não constituirá novação, devendo ser interpretada como mera liberalidade, não implicando, portanto, na desistência de exigir o cumprimento das disposições aqui contidas ou do direito de requerer futuramente a total execução de cada uma das obrigações estabelecidas neste contrato, bem como de pleitear perdas e danos.

15.9 As comunicações entre as PARTES contratantes, relacionadas com o acompanhamento e controle do presente contrato, serão sempre feitas por escrito ou por e-mail.

15.10 O término do prazo contratual, a resolução, a resilição ou a rescisão deste contrato não afeta a responsabilidade das PARTES no que tange ao sigilo a ser observado em face deste contrato, bem como eventuais ressarcimentos relativos às obrigações de cunho trabalhista, fiscal, cível ou previdenciária, ou ainda dos danos causados a terceiros decorrentes de culpa ou dolo da outra PARTE, seus empregados, prepostos e/ou sócios.


CLÁUSULA DÉCIMA SEXTA – DA ELEIÇÃO DO FORO


16.1 Fica eleito o foro da cidade de Diamantina/MG, para dirimir quaisquer divergências provenientes do presente contrato, com renúncia a quaisquer outros por mais privilegiados que sejam.

E por estarem assim justos e contratados, assinam o presente contrato em 02 (duas) vias de igual teor para o mesmo efeito, juntamente com as duas testemunhas abaixo, declarando ser perfeitamente conhecedores das condições e termos do presente, aceitando-os na forma como se encontram redigidos.


Diamantina/MG, 23 de novembro de 2023.

LOCADORA:____________________________	
LOCATÁRIA:____________________________



TESTEMUNHAS:




Nome:____________________________	Nome:____________________________
CPF:____________________________	CPF:____________________________
  `;
};


// Documento PDF com o contrato gerado
const MeuDocumento = (params) => {
  const contratoTexto = gerarTextoContratoP1(params);
  const contratoTexto2 = gerarTextoContratoP2(params);
  const nomemaquina = params.nomemaquina;
  const numserie = params.numserie;
  console.log(nomemaquina, numserie);
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
            <Text style={styles.tableCell}>QUANTIDADE</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>EQUIPAMENTO</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>NÚMERO DE SÉRIE</Text>
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
            <Text style={styles.tableCell}>1</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>{nomemaquina}</Text>
          </View>
          <View style={styles.tableCol}>
            <Text style={styles.tableCell}>{numserie}</Text>
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
  const navigate = useNavigate();

  const home = () => {
    navigate('/home');
  };

  return (
    <div>
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

      //Locatario 
        nomelocatario = {state?.nomelocatario || ""}
        cnpjlocatario = {state?.cnpjlocatario || ""}
      //Locatario | Endereço
        logradourolocatario = {state?.logradouroLocatario || ""}
        cidadelocatario = {state?.cidadeLocatario || ""}
        uflocatario = {state?.ufLocatario || ""}
      //Locatario | Socio
        nomesociolocatario = {state?.nomesociolocatario || ""} 
        nacionalidadesociolocatario = {state?.nacionalidadesociolocatario || ""} 
        estadocivilsociolocatario = {state?.estadocivilsociolocatario || ""} 
        cpfsociolocatario = {state?.cpfsociolocatario || ""} 
        logradourosociolocatario = {state?.logradouroSocioLocatario || ""} 
        cidadesociolocatairo = {state?.cidadeSocioLocatario || ""} 
        ufsociolocatario = {state?.ufSocioLocatario || ""}

      //Maquina
        nomemaquina = {state?.nomemaquina || ""}
        numserie = {state?.numserie || ""}
      />
    </PDFViewer>
    <button onClick={home}>Voltar para a página inicial</button>
    </div>
  );
};

export default CPDF;
