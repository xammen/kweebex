export const GAME = {
  name: "Hytale",
  developer: "Hypixel Studios",
  website: "https://hytale.com",
  
  releaseStatus: "unreleased" as const,
  
  contentTypes: [
    { id: "mod", label: "Mod", icon: "puzzle" },
    { id: "modpack", label: "Modpack", icon: "package" },
    { id: "resourcepack", label: "Resource Pack", icon: "palette", disabled: true },
    { id: "map", label: "Map", icon: "map", disabled: true },
    { id: "shader", label: "Shader", icon: "sun", disabled: true },
    { id: "datapack", label: "Datapack", icon: "database", disabled: true },
  ],

  versions: [
    { id: "placeholder-1.0", label: "1.0 (Placeholder)", stable: true },
    { id: "placeholder-beta", label: "Beta (Placeholder)", stable: false },
  ],

  modLoaders: [
    { id: "hytale", label: "Hytale Native", default: true },
  ],

  installPaths: {
    windows: "%APPDATA%/Hytale",
    macos: "~/Library/Application Support/Hytale", 
    linux: "~/.local/share/Hytale",
  },

  fileExtensions: {
    mod: [".hymod", ".zip"],
    modpack: [".mrpack", ".zip"],
    resourcepack: [".hyrp", ".zip"],
    map: [".hymap", ".zip"],
  },
} as const;

export type Game = typeof GAME;
export type ContentType = typeof GAME.contentTypes[number];
export type GameVersion = typeof GAME.versions[number];
