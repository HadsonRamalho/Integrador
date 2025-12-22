import {
	Card,
	CardContent,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import Layout from "@/layouts/default";

export function Login() {
	return (
		<Layout>
			<div className="flex items-center justify-center">
				<Card className="p-4">
					<CardHeader className="p-2">
						<CardTitle className="text-3xl text-center">MaqExpress</CardTitle>
						<span>Faça login para continuar</span>
					</CardHeader>
					<CardContent>
						<Label>E-mail</Label>
						<Input placeholder="Informe seu e-mail" type="email" />
						<Label>Senha</Label>
						<Input placeholder="Informe sua senha" type="password" />
					</CardContent>
					<CardFooter>
						<span>
							Não possui uma conta?{" "}
							<a className="hover:underline" href="/register">
								Clique aqui para se cadastrar
							</a>
						</span>
					</CardFooter>
				</Card>
			</div>
		</Layout>
	);
}
