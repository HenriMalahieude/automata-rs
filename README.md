# Automata Explorations
An excuse to learn rust while I'm at it

## 1-Dimensional Automata
1 Row of squares, with a kernel
Starting conditions: single square in middle

Interesting findings:
* Symmetric Self-Exclusive Automata
    * 1 Neighbor View
        * Stabilize to a fully empty/filled board
        * One empty square moves to one side
    * 2 Neighbor View
        * Ruleset 58750: Semi-random chaos, stabilizes into horizontal diagonals
        * Ruleset 772: Ordered, Stable triangular fractal pattern. Recognizable wolfram rule
        * Ruleset 34934: Semi-Random Chaos, stablizes triangular pattern into alternating pattern
        * Ruleset 30996: Ordered, Stable triangular alternating slants
        * Ruleset 12550: Ordered, Stable triangles within triangles. Recognizable wolfram rule
        * Ruleset 12947: Ruleset 772, but with alternating empty and full rows
        * Ruleset 8108: Ruleset 12550 but not pure empty/full, slants form triangular patterns
