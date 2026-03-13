// This file was updated manually to match backend/src/domain/user.rs
export type CreateUser = {
  name: string;
  email: string;
  password: string | null;
  contact: string | null;
  role: string | null;
};
