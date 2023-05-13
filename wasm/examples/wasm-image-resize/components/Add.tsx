"use client";
import { WASMContext } from "@/contexts/wasm-context";
import { useContext } from "react";

type Props = {
  arg1: number;
  arg2: number;
};

export default function Add({ arg1, arg2 }: Props) {
  const ctx = useContext(WASMContext);

  if (!ctx.wasm) {
    return <></>;
  }

  return (
    <>
      The sum of {arg1} + {arg2} is {ctx.wasm.add(arg1, arg2)}
    </>
  );
}
