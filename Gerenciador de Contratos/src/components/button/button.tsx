import { ButtonHTMLAttributes } from "react";

interface ButtonProps
  extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, "children"> {
  children: React.ReactNode;
  onClick: () => void;
}

export default function ButtonComponent({ children, ...rest }: ButtonProps) {
  return <button {...rest}>{children}</button>;
}
