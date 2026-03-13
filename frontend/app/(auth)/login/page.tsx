"use client";

import { useState } from "react";
import { useLogin } from "@/hooks/useAuth";
import { useRouter } from "next/navigation";

export default function LoginPage() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const loginMutation = useLogin();
  const router = useRouter();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    loginMutation.mutate(
      { email, password },
      {
        onSuccess: () => {
          router.push("/dashboard"); // Or wherever you want to redirect
        },
      }
    );
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-[#707066] p-4 font-sans">
      <div className="w-full max-w-md bg-gray-200 shadow-2xl rounded-3xl p-8 border border-neutral-200">
        
        <div className="text-center mb-10">
          <h1 className="text-3xl font-extrabold text-neutral-950 tracking-tight mb-2">
            Welcome Back
          </h1>
          <p className="text-neutral-600 text-sm">
            Enter your credentials to access your account
          </p>
        </div>

        {loginMutation.isError && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 text-red-600 text-sm rounded-xl">
            { (loginMutation.error as any)?.response?.data || "Invalid credentials. Please try again." }
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-6">
          <div className="space-y-4">
            <div>
              <label htmlFor="email" className="sr-only">Email</label>
              <input 
                type="email"
                id="email"
                name="email"
                required
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                placeholder="name@example.com" 
                className="w-full bg-neutral-50 border-neutral-200 border rounded-xl p-4 text-neutral-950 placeholder-neutral-400 focus:outline-none focus:ring-2 focus:ring-neutral-900 transition-all"
              />
            </div>

            <div>
              <label htmlFor="password" className="sr-only">Password</label>
              <input 
                type="password"
                id="password" 
                name="password"
                required
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="••••••"
                className="w-full bg-neutral-50 border-neutral-200 border rounded-xl p-4 text-neutral-950 placeholder-neutral-400 focus:outline-none focus:ring-2 focus:ring-neutral-900 transition-all"
              />
            </div>
          </div>

          <button
            type="submit"
            disabled={loginMutation.isPending}
            className="w-full bg-neutral-950 text-white font-bold py-4 rounded-2xl hover:bg-neutral-800 active:scale-[0.98] transition-all duration-200 shadow-lg disabled:opacity-60 disabled:cursor-not-allowed"
          >
            {loginMutation.isPending ? "Logging in..." : "Login"}
          </button>
        </form>

        <div className="mt-8 text-center">
          <p className="text-neutral-600 text-sm">
            Don&apos;t have an account?{" "}
            <a href="/registration" className="text-neutral-950 font-semibold hover:underline">
              Register
            </a>
          </p>
        </div>
      </div>
    </div>
  );
}