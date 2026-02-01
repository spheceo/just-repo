import { env } from "@/lib/env";

export default async function Home() {
  const data = await fetch(`${env.NEXT_PUBLIC_API_URL}`);
  const res = await data.json();

  return (
    <div className="flex flex-col justify-center items-center gap-2 h-dvh">
      <h1 className="text-3xl font-semibold">Next JS + Rust + Turborepo</h1>
      <p>{res.message}</p>
    </div>
  );
}
