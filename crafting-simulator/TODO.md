things to try:

- be able to make combo actions (like Observe -> Focused Synthesis which almost always appear together)
  - is there a nice way to model stuff like buffs running out halfway through a step? maybe combo steps just need to be something that the generator knows about, rather than the underlying simulation.
- generate a random sequence of steps, then apply them to a recipe
- generic algorithm-style mutation steps (swap, insert, delete, etc)
  - have a cost function which determines how good a sequence is
    - eg higher progress/quality relative to the recipe's targets, lower durability loss, etc
    - at each stage make sure we keep the best sequence that doesn't fail the craft, but also keep some other sequences that do well but fail, since they might get mutated into successful sequences
- convert a sequence of steps to a macro

things that we aren't considering (yet?)

- actions that require procs (eg Intensive Synthesis) chance-based actions (eg Rapid Synthesis)
- food and medicine (we'll just assume they're baked into the basic stats for now)
- lower level recipes (having a higher crafter level gives a bonus to recipe progression / quality increases)
- minimum required stats for a craft
- action potency changes from earlier levels (eg a lower-level crafter has 100 potency for basic synthesis rather than 120)
- crafting state procs (excellent/poor/good/etc)
