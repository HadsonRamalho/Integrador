import Layout from "@/layouts/default";
import "@/components/machine/machine.css";

import {
  Document,
  Page,
  Text,
  View,
  StyleSheet,
  pdf,
} from "@react-pdf/renderer";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";

export default function PdfExample() {
    const styles = StyleSheet.create({
        page: {
          flexDirection: "row",
          backgroundColor: "#97cfb3",
          width: '400px',
          height: '400px'
        },
        section: {
          margin: 10,
          padding: 10,
          flexGrow: 1,
          backgroundColor: '#54c78d'
        },
      });

  const MyDocument = () => (
    <Document>
      <Page size="A4" style={styles.page}>
        <View style={styles.section}>
          <Text>Section #1</Text>
        </View>
        <View style={styles.section}>
          <Text>Section #2</Text>
        </View>
      </Page>
    </Document>
  );

  const handleDownload = async () => {
    const blob = await pdf(<MyDocument />).toBlob();
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "documento.pdf";
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  return (
    <Layout>
      <main className="mt-10">
        <div className="mt-40 mb-40">
          <Card>
            <CardHeader>Documento:</CardHeader>
            <CardContent className="bg-gray-400 m-4">
              <MyDocument />
            </CardContent>
            <CardFooter>
              <Button className="ml-10" onClick={handleDownload}>
                Baixar PDF
              </Button>
            </CardFooter>
          </Card>
        </div>
      </main>
    </Layout>
  );
}
