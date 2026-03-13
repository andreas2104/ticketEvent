"use client";

import { useState } from "react";
import { useRegister } from "@/hooks/useAuth";
import { useRouter } from "next/navigation";
import Link from "next/link";

export default function RegistrationPage() {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [contact, setContact] = useState("");
  const [password, setPassword] = useState("");
  const [success, setSuccess] = useState(false);
  
  const registerMutation = useRegister();
  const router = useRouter();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    registerMutation.mutate(
      { name, email, contact, password, role: "user" },
      {
        onSuccess: () => {
          setSuccess(true);
          setTimeout(() => {
            router.push("/login");
          }, 2000);
        },
      }
    );
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-[#707066] p-4 font-sans">
      <div className="w-full max-w-md bg-gray-200 shadow-2xl rounded-3xl p-8 border border-neutral-200">
        
        <div className="text-center mb-10">
          <h1 className="text-3xl font-extrabold text-neutral-950 tracking-tight mb-2">
            Create Account
          </h1>
          <p className="text-neutral-600 text-sm">
            Join us to start managing your tickets
          </p>
        </div>

        {registerMutation.isError && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 text-red-600 text-sm rounded-xl">
            { (registerMutation.error as any)?.response?.data || "Registration failed. Please try again." }
          </div>
        )}

        {success && (
          <div className="mb-6 p-4 bg-green-50 border border-green-200 text-green-600 text-sm rounded-xl text-center">
            Account created successfully! Redirecting to login...
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-4">
            <div>
              <label htmlFor="name" className="sr-only">Full Name</label>
              <input 
                type="text"
                id="name"
                name="name"
                required
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="Full Name" 
                className="w-full bg-neutral-50 border-neutral-200 border rounded-xl p-4 text-neutral-950 placeholder-neutral-400 focus:outline-none focus:ring-2 focus:ring-neutral-900 transition-all"
              />
            </div>

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
              <label htmlFor="contact" className="sr-only">Contact</label>
              <input 
                type="text"
                id="contact"
                name="contact"
                value={contact}
                onChange={(e) => setContact(e.target.value)}
                placeholder="Contact (e.g. 034 34 344 34)" 
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
            disabled={registerMutation.isPending || success}
            className="w-full bg-neutral-950 text-white font-bold py-4 rounded-2xl hover:bg-neutral-800 active:scale-[0.98] transition-all duration-200 shadow-lg disabled:opacity-60 disabled:cursor-not-allowed mt-6"
          >
            {registerMutation.isPending ? "Creating Account..." : "Register"}
          </button>
        </form>

        <div className="mt-8 text-center">
          <p className="text-neutral-600 text-sm">
            Already have an account?{" "}
            <Link href="/login" className="text-neutral-950 font-semibold hover:underline">
              Login
            </Link>
          </p>
        </div>
      </div>
    </div>
  );
}