import { apiClient } from "../apiClient";
import { LoginRequest } from "../../types/LoginRequest";
import { AuthResponse } from "../../types/AuthResponse";
import { CreateUser } from "../../types/CreateUser";

export const authApi = {
  login: async (data: LoginRequest): Promise<AuthResponse> => {
    const response = await apiClient.post<AuthResponse>("/login", data);
    return response.data;
  },
  
  register: async (data: CreateUser): Promise<number> => {
    const response = await apiClient.post<number>("/register", data);
    return response.data;
  },
};
