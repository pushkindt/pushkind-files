import { useEffect, useRef } from "react";

type BootstrapInstance = {
  dispose?: () => void;
};

type BootstrapConstructor = new (element: Element) => BootstrapInstance;

type BootstrapNamespace = {
  Tooltip?: BootstrapConstructor;
  Popover?: BootstrapConstructor;
};

declare global {
  interface Window {
    bootstrap?: BootstrapNamespace;
  }
}

export function initializeBootstrapSurface(root: ParentNode) {
  const bootstrap = window.bootstrap;
  const disposables: BootstrapInstance[] = [];

  if (!bootstrap) {
    return () => {};
  }

  const Tooltip = bootstrap.Tooltip;
  if (Tooltip) {
    root.querySelectorAll('[data-bs-toggle="tooltip"]').forEach((element) => {
      disposables.push(new Tooltip(element));
    });
  }

  const Popover = bootstrap.Popover;
  if (Popover) {
    root.querySelectorAll('[data-bs-toggle="popover"]').forEach((element) => {
      disposables.push(new Popover(element));
    });
  }

  return () => {
    for (const disposable of disposables) {
      disposable.dispose?.();
    }
  };
}

export function useBootstrapSurface<T extends HTMLElement>() {
  const ref = useRef<T | null>(null);

  useEffect(() => {
    if (!ref.current) {
      return;
    }

    return initializeBootstrapSurface(ref.current);
  }, []);

  return ref;
}
