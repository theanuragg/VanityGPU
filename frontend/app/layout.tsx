import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "VanityGPU | Secure Distributed Vanity Search",
  description: "Generate Solana vanity addresses securely using GPU compute with X25519 encryption and Zeroize memory safety.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="antialiased">
        {children}
      </body>
    </html>
  );
}
