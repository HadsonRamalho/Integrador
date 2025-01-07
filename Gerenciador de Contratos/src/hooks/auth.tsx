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
    const data = await loginUser(email, password);

    localStorage.setItem("user", JSON.stringify(data));
    setUser(data);
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
