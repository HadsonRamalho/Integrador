import React from "react";
import { FcGoogle } from "react-icons/fc";

type GoogleLoginButtonProps = {
  onClick: () => void;
};

const GoogleLoginButton: React.FC<GoogleLoginButtonProps> = ({ onClick }) => {
  return (
    <button
      onClick={onClick}
      className="flex items-center justify-center gap-2 px-8 py-2 border border-gray-300 rounded-lg shadow-sm bg-white hover:bg-gray-100 transition duration-200"
    >
      <FcGoogle className="text-xl" />
      <span className="text-sm font-medium text-gray-600">
        Entrar com o Google
      </span>
    </button>
  );
};

export default GoogleLoginButton;
