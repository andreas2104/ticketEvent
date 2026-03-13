// This file was updated manually to match backend/src/domain/user.rs
export type User = {
  id: number;
  name: string;
  email: string;
  contact: string | null;
  google_id: string | null;
  avatar_url: string | null;
  role: string;
  created_at: string | null;
};
