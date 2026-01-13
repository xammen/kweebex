/**
 * Hytale Game Configuration
 * 
 * Game-specific constants that differ from Minecraft/Modrinth.
 * This file will be updated as Hytale releases and we learn more
 * about its modding system.
 */

export const GAME = {
  name: "Hytale",
  developer: "Hypixel Studios",
  website: "https://hytale.com",
  
  releaseStatus: "unreleased" as const,
  
  contentTypes: [
    { id: "mod", label: "Mod", icon: "puzzle" },
    { id: "resourcepack", label: "Resource Pack", icon: "palette" },
    { id: "map", label: "Map", icon: "map" },
    { id: "prefab", label: "Prefab", icon: "cube" },
    { id: "model", label: "Model", icon: "box" },
  ],

  versions: [
    { id: "placeholder-1.0", label: "1.0 (Placeholder)", stable: true },
    { id: "placeholder-beta", label: "Beta (Placeholder)", stable: false },
  ],

  modLoaders: [
    { id: "native", label: "Native (Hytale API)", default: true },
  ],

  installPaths: {
    windows: "%APPDATA%/Hytale",
    macos: "~/Library/Application Support/Hytale", 
    linux: "~/.local/share/Hytale",
  },

  fileExtensions: {
    mod: [".hymod", ".zip"],
    resourcepack: [".hyrp", ".zip"],
    map: [".hymap", ".zip"],
  },
} as const;

export type Game = typeof GAME;
export type ContentType = typeof GAME.contentTypes[number];
export type GameVersion = typeof GAME.versions[number];
