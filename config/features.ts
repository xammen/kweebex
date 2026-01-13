export const FEATURES = {
  shaders: false,
  datapacks: false,
  resourcepacks: false,
  plugins: false,
  mods: true,
  modpacks: true,

  loaderFabric: false,
  loaderForge: false,
  loaderQuilt: false,
  loaderNeoforge: false,
  loaderLiteloader: false,
  loaderRift: false,
  loaderHytaleNative: true,

  serverHosting: false,
} as const;

export function isProjectTypeEnabled(type: string): boolean {
  switch (type.toLowerCase()) {
    case "mod": return FEATURES.mods;
    case "modpack": return FEATURES.modpacks;
    case "shader": return FEATURES.shaders;
    case "datapack": return FEATURES.datapacks;
    case "resourcepack": return FEATURES.resourcepacks;
    case "plugin": return FEATURES.plugins;
    default: return false;
  }
}

export function isLoaderEnabled(loader: string): boolean {
  switch (loader.toLowerCase()) {
    case "hytale": return FEATURES.loaderHytaleNative;
    case "fabric": return FEATURES.loaderFabric;
    case "forge": return FEATURES.loaderForge;
    case "quilt": return FEATURES.loaderQuilt;
    case "neoforge": return FEATURES.loaderNeoforge;
    case "liteloader": return FEATURES.loaderLiteloader;
    case "rift": return FEATURES.loaderRift;
    default: return false;
  }
}

export function getEnabledProjectTypes(): string[] {
  const types: string[] = [];
  if (FEATURES.mods) types.push("mod");
  if (FEATURES.modpacks) types.push("modpack");
  if (FEATURES.shaders) types.push("shader");
  if (FEATURES.datapacks) types.push("datapack");
  if (FEATURES.resourcepacks) types.push("resourcepack");
  if (FEATURES.plugins) types.push("plugin");
  return types;
}

export function getEnabledLoaders(): string[] {
  const loaders: string[] = [];
  if (FEATURES.loaderHytaleNative) loaders.push("hytale");
  if (FEATURES.loaderFabric) loaders.push("fabric");
  if (FEATURES.loaderForge) loaders.push("forge");
  if (FEATURES.loaderQuilt) loaders.push("quilt");
  if (FEATURES.loaderNeoforge) loaders.push("neoforge");
  if (FEATURES.loaderLiteloader) loaders.push("liteloader");
  if (FEATURES.loaderRift) loaders.push("rift");
  return loaders;
}
