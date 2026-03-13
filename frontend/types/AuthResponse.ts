import { User } from "./User";

// This file was updated manually to match backend/src/domain/login.rs
export type AuthResponse = {
  token: string;
  user: User;
};
