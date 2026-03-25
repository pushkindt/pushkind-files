import type { ReactNode } from "react";

export type FlashMessage = {
  id: string;
  message: string;
  category: "primary" | "success" | "warning" | "danger";
};

export function FlashStack({
  flashes,
  fallback,
}: {
  flashes?: FlashMessage[];
  fallback?: ReactNode;
}) {
  if (!flashes?.length) {
    return fallback ? <>{fallback}</> : null;
  }

  return (
    <div className="mb-3" id="flashMessages">
      {flashes.map((flash) => (
        <div
          key={flash.id}
          className={`alert alert-${flash.category} alert-dismissible`}
          role="alert"
        >
          {flash.message}
          <button
            type="button"
            className="btn-close"
            data-bs-dismiss="alert"
            aria-label="Close"
          />
        </div>
      ))}
    </div>
  );
}
