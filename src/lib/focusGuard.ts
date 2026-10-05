import { lockCollapse, unlockCollapse } from "../stores/app";

const FOCUS_RELEASE_DELAY_MS = 160;

/**
 * Keeps focus-loss collapse from interrupting an active editor or native picker.
 * The delayed release lets the window blur handler observe the lock first.
 */
export function protectFromFocusLoss(node: HTMLElement): { destroy: () => void } {
  let locked = false;
  let releaseHandle: ReturnType<typeof setTimeout> | null = null;

  function hold(): void {
    if (releaseHandle !== null) {
      clearTimeout(releaseHandle);
      releaseHandle = null;
    }
    if (!locked) {
      locked = true;
      lockCollapse();
    }
  }

  function releaseSoon(): void {
    if (releaseHandle !== null) clearTimeout(releaseHandle);
    releaseHandle = setTimeout(() => {
      releaseHandle = null;
      if (locked) {
        locked = false;
        unlockCollapse();
      }
    }, FOCUS_RELEASE_DELAY_MS);
  }

  node.addEventListener("focusin", hold);
  node.addEventListener("focusout", releaseSoon);

  return {
    destroy(): void {
      node.removeEventListener("focusin", hold);
      node.removeEventListener("focusout", releaseSoon);
      if (releaseHandle !== null) clearTimeout(releaseHandle);
      if (locked) unlockCollapse();
    },
  };
}
