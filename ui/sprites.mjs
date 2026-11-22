// The icons live in each design file's sprite, where a browser opening the
// file from disk can reach them. The build copies those sprites into the
// app's page, so the design files and the app draw the same icons.

const SPRITE = /<svg\b[^>]*>\s*(?:<symbol\b[\s\S]*?<\/symbol>\s*)+<\/svg>/;
const SYMBOL_ID = /<symbol\b[^>]*\bid="([^"]+)"/g;

/**
 * Collects the icon sprite of each design file into one block of markup.
 *
 * @param {ReadonlyArray<{ name: string, html: string }>} files The design
 *   files, each with its name and its text.
 * @returns {string} The sprites, one per file, joined by a newline.
 * @throws {Error} When a file holds no sprite, or when two icons share an id.
 */
export function collectSprites(files) {
  const owners = new Map();
  const sprites = [];
  for (const { name, html } of files) {
    const match = SPRITE.exec(html);
    if (match === null) {
      throw new Error(`${name} holds no icon sprite`);
    }
    for (const [, id] of match[0].matchAll(SYMBOL_ID)) {
      const owner = owners.get(id);
      if (owner !== undefined) {
        throw new Error(`icon id "${id}" is in both ${owner} and ${name}`);
      }
      owners.set(id, name);
    }
    sprites.push(match[0]);
  }
  return sprites.join("\n");
}
