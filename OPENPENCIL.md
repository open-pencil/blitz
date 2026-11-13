# Blitz for OpenPencil

Branch `openpencil/blitz` of open-pencil/blitz: Blitz as [OpenPencil](https://github.com/open-pencil/open-pencil) builds it into its HTML engine (`packages/html-engine`), which lays HTML out to import it as layers. It changes only through reviewed pull requests; the OpenPencil section of `AGENTS.md` has the rules.

- **Base:** DioxusLabs/blitz#1370 at `375fe2dd`, which lays CSS tables out with Taffy's table layout instead of a grid, so rows, row groups, and captions have boxes and columns are sized as browsers size them. Move it when #1370 merges, to the upstream commit that holds it.
- **Taffy:** the workspace `[patch]` in `Cargo.toml` takes Taffy from open-pencil/taffy's `openpencil/taffy`, which carries DioxusLabs/taffy#1380 (table layout) and DioxusLabs/taffy#1361 (requested baselines) that the pull requests here need. open-pencil/open-pencil patches Taffy to the same branch in its own workspace, so HTML import and auto layout share one Taffy; move both revs together.
- **No releases:** OpenPencil depends on a commit of this branch as a Cargo git dependency, so nothing is built or published here.

## Upstream pull requests carried

Each is dropped here once upstream has it and the base includes it.

- DioxusLabs/blitz#1279: inline layout requests atomic-inline baselines from Taffy, so an inline-block or inline-flex box inside a flex or grid container sits on its line; without it such a box sits 6 px low and every row below it shifts. Merged into the base by hand, since #1370 moved the inline-box loop into a closure.
