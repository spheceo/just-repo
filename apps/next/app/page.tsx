import { env } from "@/lib/env";

export default async function Home() {
  const apiUrl = env.NEXT_PUBLIC_API_URL;

  if (!apiUrl) {
    return (
      <div className="flex flex-col justify-center items-center gap-2 h-dvh">
        <h1 className="text-3xl font-semibold">Next JS + Rust + Turborepo</h1>
        <p>Set NEXT_PUBLIC_API_URL to fetch the API response.</p>
      </div>
    );
  }

  const data = await fetch(apiUrl);
  const res = await data.json();

  return (
    <div className="flex flex-col justify-center items-center gap-2 h-dvh">
      <h1 className="text-3xl font-semibold">Next JS + Rust + Turborepo</h1>
      <p>{res.message}</p>
    </div>
  );
}
