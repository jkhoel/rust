import Add from "@/components/Add";
import Greeting from "@/components/Greeting";
import { WASMContextProvider } from "@/contexts/wasm-context";

export default function Home({ children }: { children: React.ReactNode }) {
  return (
    <main className="flex min-h-screen flex-col p-24">
      <h1>Welcome!</h1>
      <WASMContextProvider>
        <p>
          <Greeting name="NextJS and WebAssembly" />
        </p>
        <Add arg1={2} arg2={4} />
      </WASMContextProvider>
    </main>
  );
}
