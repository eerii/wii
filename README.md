# wii 🌳

sway utils written in rust

### usage

- `wii next`: go to next workspace
- `wii prev`: go to previous workspace
- `wii move next`: move window to next workspace
- `wii move prev`: move window to previous workspace

the rules for determining the next/previous valid workspace are:

- find the next closest workspace or empty gap in the same output
- if there is no valid location:
  - for next, create a new workspace at the end
  - for prev, stay in the same place

this makes it possible to move seamlessly without explicitly creating new workspaces

### todo

- [x] basic movement
- [x] respect the output each workspace is assigned to
- [ ] names, subspaces
- [ ] other useful tools
