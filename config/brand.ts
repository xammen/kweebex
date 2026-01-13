/**
 * Kweebex Brand Configuration
 * 
 * Centralized branding constants for the entire project.
 * This file makes it easy to:
 * - Update branding in one place
 * - Merge upstream Modrinth updates without conflicts
 * - Maintain consistent branding across all apps
 */

export const BRAND = {
  // Core Identity
  name: "Kweebex",
  shortName: "Kweebex",
  tagline: "Discover and share Hytale mods",
  description: "The open source modding platform for Hytale",
  
  // Original project (for attribution)
  basedOn: {
    name: "Modrinth",
    url: "https://modrinth.com",
    repo: "https://github.com/modrinth/code"
  },

  // URLs
  urls: {
    website: "https://kweebex.com",
    api: "https://api.kweebex.com",
    cdn: "https://cdn.kweebex.com",
    docs: "https://docs.kweebex.com",
    discord: "https://discord.gg/kweebex",
    github: "https://github.com/xammen/kweebex",
    twitter: "https://twitter.com/kweebex",
  },

  // Colors (Hytale-inspired palette)
  colors: {
    primary: "#00A8E8",      // Hytale blue
    primaryDark: "#0077B6",
    secondary: "#FF6B35",    // Accent orange
    success: "#10B981",
    warning: "#F59E0B", 
    error: "#EF4444",
    
    // Background shades
    background: {
      dark: "#0F172A",
      medium: "#1E293B",
      light: "#334155",
    }
  },

  // Legal
  legal: {
    companyName: "Kweebex",
    copyrightYear: new Date().getFullYear(),
  },

  // Social/SEO
  social: {
    defaultOgImage: "/images/og-default.png",
    twitterHandle: "@kweebex",
  }
} as const;

export type Brand = typeof BRAND;
