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

  // Colors (Hytale official palette)
  colors: {
    primary: "#15243a",      // Hytale dark blue (main brand color)
    primaryLight: "#1e3a5f", // Lighter variant
    primaryDark: "#0d1829",  // Darker variant
    accent: "#e8b923",       // Hytale gold/yellow accent
    accentHover: "#f5c842",
    secondary: "#3d7ea6",    // Hytale teal/cyan
    success: "#4ade80",
    warning: "#fbbf24", 
    error: "#f87171",
    
    // Text colors
    text: {
      primary: "#ffffff",
      secondary: "#94a3b8",
      muted: "#64748b",
    },
    
    // Background shades (Hytale dark theme)
    background: {
      dark: "#0d1829",       // Darkest
      medium: "#15243a",     // Main Hytale blue
      light: "#1e3a5f",      // Lighter panels
      card: "#1a3050",       // Card backgrounds
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
