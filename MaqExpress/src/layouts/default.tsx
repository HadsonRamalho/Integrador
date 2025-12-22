import Footer from "../components/footer";
import Header from "../components/header";
import "@/layouts/index.css";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <Header />
      <div className="navbar">
      {children}
      </div>
      <Footer />
    </>
  );
}
