import LoggedLayout from "@/layouts/logged-layout";

function Logged() {
  return (
    <>
      <LoggedLayout>
        <main>
          <h3>Esta página possui um layout para usuários logados</h3>
        </main>
      </LoggedLayout>
    </>
  );
}

export default Logged;
