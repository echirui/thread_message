import { createContext } from 'react';

export interface User {
  id: number;
  github_id: number;
  username: string;
  avatar_url: string | null;
  created_at: string;
}

export interface AuthContextType {
  user: User | null;
  loading: boolean;
  login: () => void;
  logout: () => Promise<void>;
  isAuthenticated: boolean;
}

export const AuthContext = createContext<AuthContextType | undefined>(undefined);