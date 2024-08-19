import type { Metadata } from "next";
import "./styles/index.scss";

import { makeStore } from "./lib/store";
import StoreProvider from "./lib/StoreProvider";

export const metadata: Metadata = {
  title: "Caban: lasdf",
  description: " ыва",
};

export default function RootLayout({ children, }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="ru">
      <body>
        <StoreProvider>
          {children}
        </StoreProvider>
      </body>
    </html>
  );
}
