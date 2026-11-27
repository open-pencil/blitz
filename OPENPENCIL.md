# Blitz for OpenPencil

Branch `openpencil/blitz` of open-pencil/blitz: Blitz as [OpenPencil](https://github.com/open-pencil/open-pencil) builds it into its HTML engine (`packages/html-engine`), which lays HTML out to import it as layers. It changes only through reviewed pull requests; the OpenPencil section of `AGENTS.md` has the rules.

- **Base:** DioxusLabs/blitz#1370 at `375fe2dd`, which lays CSS tables out with Taffy's table layout instead of a grid, so rows, row groups, and captions have boxes and columns are sized as browsers size them. Move it when #1370 merges, to the upstream commit that holds it.
- **Taffy:** the workspace `[patch]` in `Cargo.toml` takes Taffy from open-pencil/taffy's `openpencil/taffy`, which carries DioxusLabs/taffy#1380 (table layout) and DioxusLabs/taffy#1361 (requested baselines) that the pull requests here need. open-pencil/open-pencil patches Taffy to the same branch in its own workspace, so HTML import and auto layout share one Taffy; move both revs together.
- **No releases:** OpenPencil depends on a commit of this branch as a Cargo git dependency, so nothing is built or published here.

## Upstream pull requests carried

Each is dropped here once upstream has it and the base includes it.

- DioxusLabs/blitz#1279: inline layout requests atomic-inline baselines from Taffy, so an inline-block or inline-flex box inside a flex or grid container sits on its line; without it such a box sits 6 px low and every row below it shifts. Merged into the base by hand, since #1370 moved the inline-box loop into a closure.
- DioxusLabs/blitz#1396: a table's `border` attribute draws the borders it asks for on the table and its cells. The default stylesheet's rules for it matched through Gecko's `:-moz-table-border-nonzero`, which Stylo's Servo parser drops, so neither drew and every box of a bordered table sat a pixel or two off. Cherry-picked unchanged.
- DioxusLabs/blitz#799, in part: `:focus-visible` matches the `FOCUSRING` element-state bit, which focus already sets, and the default stylesheet draws the focus outline on `:focus-visible` as browsers do. OpenPencil sets the bit on the elements a page shows focused. The rest of #799, which sets the bit only for keyboard focus, is left out: it is written against a Blitz from before the focus system it changes was reworked, and OpenPencil focuses no element through events. Test `focus_visible_matches_focused_element`.

## Fixes of our own

Each also goes upstream, following Blitz's CONTRIBUTING.MD, and is dropped here once the base includes it.

- Refuse a stylesheet `@import` whose URL does not resolve instead of panicking (`net.rs`), as in a document without a base URL, which OpenPencil's CLI and MCP server lay out; test `unresolvable_import`. Upstream: DioxusLabs/blitz#1563.
- Fix broken intra-doc links in blitz-dom, blitz-vibey-script, and the `transparent` example, which `cargo doc` with `-D warnings` refuses. Upstream: DioxusLabs/blitz#1774.
- CI's Documentation job documents every workspace crate (`cargo doc --workspace --no-deps`); a bare `cargo doc` documents only the root `blitz-examples` package, which has no library, so the job built no docs. Upstream: DioxusLabs/blitz#1775.
- `BaseDocument::lazy_pseudo_style` computes the style of a lazily cascaded pseudo-element, such as `::placeholder`, of an element: the rules that target it applied over what it inherits from the element. Blitz cascades only the eager `::before` and `::after`, so OpenPencil could not draw a field's placeholder text as a page styles it; test `lazy_pseudo_style_applies_the_rules_that_target_it_over_what_it_inherits`. Upstream: not yet proposed.
