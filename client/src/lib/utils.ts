import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function isNumber(input: string): boolean {
  const regex = /^\d+$/;
  return !regex.test(input);
}
