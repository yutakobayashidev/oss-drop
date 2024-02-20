# sdk

> [!WARNING]
> This SDK system is currently in the design phase and is not yet available for use. Please help us by contributing to the whitepaper.

By using this TypeScript SDK, you can utilize the oss-drop API to easily identify users who contribute frequently to open source and integrate appropriate reward mechanisms.

## Examples

### NextAuth.js

For instance, you can incorporate a system that grants credits during account creation with NextAuth.

```ts
import { db } from "@/db/client";
import { DrizzleAdapter } from "@auth/drizzle-adapter";
import type { NextAuthConfig } from "next-auth";
import Google from "next-auth/providers/google";

export const authConfig: NextAuthConfig = {
  adapter: DrizzleAdapter(db),
  providers: [
    Google({
      authorization: {
        params: {
          prompt: "select_account",
        },
      },
      clientId: process.env.GOOGLE_CLIENT_ID,
      clientSecret: process.env.GOOGLE_CLIENT_SECRET,
    }),
  ],
  callbacks: {
    async signIn({ account, profile }) {
      if (
        account &&
        account.provider === "google" &&
        profile &&
        profile.email
      ) {
        return profile.email.endsWith("@google.com"); /
      }
      return true;
    },
  },
  pages: {
    signIn: "/login",
  },
};
```
