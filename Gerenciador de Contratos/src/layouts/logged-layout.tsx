import Footer from "../components/footer";
import Header from "../components/header";

export default function LoggedLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <>
      <Header />
      <h2>Somente pessoas com login podem acessar</h2>
      {children}
      <Footer />
    </>
  );
}
