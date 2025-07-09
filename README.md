![image](https://github.com/user-attachments/assets/715be171-bc6a-4683-bb57-06d03bb0f167)


This is my attempt, still in progress, of making a fairly comprehensive world-generator for TTRPGs, or maybe a game, or maybe just for inspiration. The end goal is a detailed world with its own geography, biomes, sense of history both geological and anthropomorphic (good word).

## CURRENT STATUS

- Core Data Structure: a `World` struct holds a grid of `Tiles`, using a flattened `Vec<Tile>`
- Perlin noise elevation: Though this is only step one, I've got a `noisegen` module using Fbm to make a heightmap.
- Terrain types: Sea, land. Eventually will hold many, many more types.
- Visualization: `Macroquad` for visualizing, with the rendering performed onto a `RenderTarget`.


## ROADMAP

I hesitate to call it a roadmap, really, but for my next steps, I'm going to add:
- River generation
- Lakes, inland seas
- Temperature, climate
- Biomes
- Randomly generated animal populations
- Any other natural feature I can think of

Plans for later:
- Seeding and simulating civilizations in time using real world population dynamics
- A database structure for events, both granular (Frank beat up Mike) and large (Civilization A toppled Civilization B)
- User interaction (Panning and zooming) ((this will be heccin HUGE))
- Saving, loading
- UI
