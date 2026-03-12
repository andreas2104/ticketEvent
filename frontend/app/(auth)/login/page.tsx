export default function LoginPage() {
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

        <form action="#" className="space-y-6">
          <div className="space-y-4">
            <div>
              <label htmlFor="email" className="sr-only">Email</label>
              <input 
                type="email"
                id="email"
                name="email"
                required
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
                placeholder="••••••"
                className="w-full bg-neutral-50 border-neutral-200 border rounded-xl p-4 text-neutral-950 placeholder-neutral-400 focus:outline-none focus:ring-2 focus:ring-neutral-900 transition-all"
              />
            </div>
          </div>

          <button
            type="submit"
            className="w-full bg-neutral-950 text-white font-bold py-4 rounded-2xl hover:bg-neutral-800 active:scale-[0.98] transition-all duration-200 shadow-lg disabled:opacity-60 disabled:cursor-not-allowed"
          >
            Login
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