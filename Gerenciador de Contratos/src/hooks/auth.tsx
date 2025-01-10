import { UserId } from "@/interfaces/user";
import { loginUser } from "@/services/api/user/user";
import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
} from "react";

interface AuthCredentials {
  email: string;
  password: string;
}

interface AuthContextData {
  user: UserId | null;
  signIn(credentials: AuthCredentials): void;
  signOut(): void;
}

const AuthContext = createContext<AuthContextData>({} as AuthContextData);

export default function AuthProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [user, setUser] = useState<UserId | null>(() => {
    const user = localStorage.getItem("user");

    if (!user) {
      return null;
    }

    const userJSON = JSON.parse(user);
    return userJSON;
  });

  const signIn = useCallback(async ({ email, password }: AuthCredentials) => {
    try{
      const data = await loginUser(email, password);

      localStorage.setItem("USER_ID", data.idusuario);
      setUser(data);
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } catch (error: any) {
      const statusCode = error.response?.status;
  
      // Lança exceções com base no código de status
      if (statusCode === 401) {
        throw new Error("Credenciais inválidas. Verifique seu e-mail e senha.");
      } else if (statusCode === 500) {
        throw new Error("Erro no servidor. Por favor, tente novamente mais tarde.");
      } else {
        throw new Error(`Erro inesperado: ${statusCode || error.message}`);
      }
    }    
  }, []);

  const signOut = useCallback(() => {
    localStorage.removeItem("user");
    setUser(null);
  }, []);

  const providerData = useMemo(() => {
    return {
      user,
      signIn,
      signOut,
    };
  }, [user, signIn, signOut]);

  return (
    <AuthContext.Provider value={providerData}>{children}</AuthContext.Provider>
  );
}

export function useAuth(): AuthContextData {
  const context = useContext(AuthContext);

  if (!context) throw new Error("useAuth must be used within an AuthProvider");

  return context;
}
