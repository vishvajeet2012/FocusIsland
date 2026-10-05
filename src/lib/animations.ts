import type { AnimationIntensity } from "../types/settings";

export function motionDuration(intensity: AnimationIntensity, full: number, reduced: number): number {
  if (intensity === "off" || systemPrefersReducedMotion()) return 0;
  return intensity === "reduced" ? reduced : full;
}

export function systemPrefersReducedMotion(): boolean {
  return typeof window !== "undefined" && window.matchMedia("(prefers-reduced-motion: reduce)").matches;
}

export function nextFrame(): Promise<void> {
  return new Promise((resolve) => requestAnimationFrame(() => resolve()));
}

export function afterPaint(): Promise<void> {
  return new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
}

export async function waitForMotion(intensity: AnimationIntensity, full: number, reduced: number): Promise<void> {
  const duration = motionDuration(intensity, full, reduced);
  if (duration > 0) await new Promise((resolve) => setTimeout(resolve, duration));
}
