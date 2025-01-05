import Layout from "@/layouts/default";
import "@/components/create-account/create-account.css";

export default function Create() {
  return (
    <Layout>
      <main>
        <div>
        <h1>crie sua conta</h1>
        <div className="create-account-box">
          <span className="pl-[45%]">Crie sua conta</span>
          <div className="create-account"></div>
        </div>
        </div>
      </main>
    </Layout>
  );
}