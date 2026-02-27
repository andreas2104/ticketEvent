# TicketApp Frontend

A modern, responsive web application for browsing events and purchasing tickets, built with **Next.js**, **React**, and **Tailwind CSS**.

## ✨ Features

- **Event Discovery**: Browse upcoming events with a clean and intuitive interface.
- **Responsive Design**: Fully optimized for mobile and desktop viewing.
- **Fast Interactions**: Seamless data fetching using React Query.
- **Modern UI**: Styled with Tailwind CSS for a premium look and feel.

## 🛠️ Tech Stack

- **Framework**: [Next.js](https://nextjs.org/) (App Router)
- **Library**: [React](https://react.dev/)
- **Styling**: [Tailwind CSS](https://tailwindcss.com/)
- **Data Fetching**: [React Query](https://tanstack.com/query/latest) & [Axios](https://axios-http.com/)
- **Language**: [TypeScript](https://www.typescriptlang.org/)

## 🏃 Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or later)
- [pnpm](https://pnpm.io/) (recommended)

### Installation

1. Clone the repository and navigate to the frontend:
   ```bash
   git clone <your-repo-url>
   cd ticketapp/frontend
   ```

2. Install dependencies:
   ```bash
   pnpm install
   ```

3. Configure environment variables:
   Create a `.env.local` file:
   ```env
   NEXT_PUBLIC_API_URL=http://localhost:4000/api
   ```

4. Start the development server:
   ```bash
   pnpm dev
   ```

The app will be available at `http://localhost:3000`.

## 📁 Structure

- `app/`: Next.js App Router pages and layouts.
- `lib/`: Shared utilities, API clients, and hooks.
- `public/`: Static assets.

## 📝 License

This project is licensed under the MIT License.
