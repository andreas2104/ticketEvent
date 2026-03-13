import { useMutation } from "@tanstack/react-query";
import { authApi } from "@/lib/api/auth";
import { LoginRequest } from "@/types/LoginRequest";
import { CreateUser } from "@/types/CreateUser";
import { AuthResponse } from "@/types/AuthResponse";

export const useLogin = () => {
  return useMutation({
    mutationFn: (data: LoginRequest) => authApi.login(data),
    onSuccess: (data: AuthResponse) => {
      localStorage.setItem("token", data.token);
      localStorage.setItem("user", JSON.stringify(data.user));
    },
  });
};

export const useRegister = () => {
  return useMutation({
    mutationFn: (data: CreateUser) => authApi.register(data),
  });
};
